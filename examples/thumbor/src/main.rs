use anyhow::Result;

use axum::{
    extrac::{Path,Extension}, 
    handler::get, 
    http::{StatusCode,HeaderMap,HeaderValue},
    AddExtensionLayer,
    Router
};
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::{percent_decode_str,percent_encode,NON_ALPHANUMERIC};
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    hash::{Hash,Hasher},
    sybc::Arc,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::{info,instrument};

mod pb;
use pb::*;

mod engine;
use engine::{Engine,Photon};
use image::ImageOutputFormat;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));


    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    let addr = "127.0.0.1:3000".parse().unwrap();

    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}

async fn generate(Path(Params {spec,url}): Path<Params>) -> Result<(String, StatusCode)> {
    let url = perccent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = Spec
                            .as_str()
                            .try_into()
                            .map_err(|_| StatusCode::BAD_REQUEST)?;

    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();
    let data = retrieve_image(&url, cache)
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut engine: Photon = data.try_into()
                                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                            
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));
    info!("Finished processing: image size {}", image.len());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));

    // 最后一行代码加分号与不加分号的区别
    Ok(headers, data.to_vec())

    // Ok(format!("url: {} \n spec: {:#?}", url, spec))
}

#[instrument(level="info", skip(cache))]
async fn retrieve_image(url: &str, cache: &Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        },
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };
}

fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500,800,resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20,20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1,spec2,spec3]);
    let s: String = image_spec.borrow().into();
    let test_iamge = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();

    println!("test url: http://localhost:3000/image/{}/{}", s, test_iamge);
}