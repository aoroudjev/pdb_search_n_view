use axum::{Json, Router, routing::get};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::body::Bytes;
use serde::{Serialize, Deserialize};

use urlencoding::encode;
use reqwest;
use reqwest::Response;

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    query_id: String,
    result_type: String,
    total_count: usize,
    result_set: Vec<Entry>,
    facets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    identifier: String,
    score: f64,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/search", get(search_handler))
        .route("/download", get(download_pdb));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct SearchParams {
    search_term: String,
}

async fn search_handler(Query(params): Query<SearchParams>) -> Result<Json<SearchResult>, StatusCode> {
    let search_term = params.search_term;
    let search_request = serde_json::json!({
          "query": {
            "type": "terminal",
            "service": "full_text",
            "parameters": {
              "value": search_term
            }
          },
          "return_type": "entry",
            "request_options": {
                "return_all_hits": true
            }
        }).to_string();
    dbg!(&search_request);


    let encoded_search_request = encode(&search_request);
    let api_url = format!("https://search.rcsb.org/rcsbsearch/v2/query?json={}", encoded_search_request);
    let client = reqwest::Client::new();

    let search_result_json = client.get(api_url)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<SearchResult>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(search_result_json))
}

#[derive(Deserialize)]
struct DownloadParams {
    pdb_id: String,
}

async fn download_pdb(Query(params): Query<DownloadParams>) -> Result<Bytes, StatusCode> {
    let pdb_id = params.pdb_id;
    let download_url = format!("https://files.rcsb.org/download/{}.pdb", pdb_id);

    let response: Result<Response, StatusCode> = reqwest::get(download_url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let pdb_data = response?.bytes().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    Ok(pdb_data?)
    // TODO: This will chane depending on the frontend process.
}
