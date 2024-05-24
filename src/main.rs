use std::collections::HashMap;
use axum::{Json, Router, routing::get};
use axum_macros::debug_handler;
use axum::body::Bytes;
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use reqwest;
use reqwest::{Client};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(uniprot_search_handler))
        .route("/data", get(alphafold_data_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchParams {
    search_term: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EntryData {
    entryType: String,
    primaryAccession: String,
    uniProtkbId: String,
    annotationScore: f32,
    organism: HashMap<String, serde_json::Value>,
    features: Vec<HashMap<String, serde_json::Value>>,
    sequence: serde_json::Value,
    comments: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResults {
    results: Vec<EntryData>,
}

#[debug_handler]
async fn uniprot_search_handler(Query(params): Query<SearchParams>) -> Result<Json<SearchResults>, StatusCode> {
    // Handler for searching through the uniprot database of entries using the given search term
    let search_term = params.search_term;
    dbg!(&search_term);

    let api_url = format!("https://rest.uniprot.org/uniprotkb/search?query={}", search_term);
    dbg!(&api_url);

    let client: Client = Client::new();
    let search_result = client.get(api_url)
        .send()
        .await
        .map_err(|e| {
            dbg!(e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .json::<SearchResults>()
        .await
        .map_err(|e| {
            dbg!(e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(search_result))
}

#[debug_handler]
async fn alphafold_data_handler(Query(params): Query<SearchParams>) -> Result<Bytes, StatusCode> {
    // Handler for retrieving protein structure data from uniprot for one specific entry
    let search_term = params.search_term;
    dbg!(&search_term);

    let api_url = format!("https://alphafold.ebi.ac.uk/files/AF-{}-F1-model_v4.pdb", search_term);
    dbg!(&api_url);

    let client: Client = Client::new();
    let search_result = client.get(api_url)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let pdb_data = search_result?.bytes().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(pdb_data)
}

