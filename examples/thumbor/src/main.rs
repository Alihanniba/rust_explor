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
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}

async fn generate(Path(Params {spec,url}): Path<Params>) -> Result<(String, StatusCode)> {
    let url = perccent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = Spec.as_str()
                                .try_into()
                                .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {} \n spec: {:#?}", url, spec))
}