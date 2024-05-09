use axum::{Json, Router, routing::get};
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};

use urlencoding::encode;
use reqwest;

#[derive(Default, Serialize, Deserialize, Debug)]
struct SearchResult {
    //TODO: Fix structure of returns
    pdb_id: String,
    name: String,
    description: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/search", get(search_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct PdbId {
    pdb_id: String,
}

async fn search_handler(Query(params): Query<PdbId>) -> Result<Json<SearchResult>, StatusCode> {
    let search_input = params.pdb_id;
    let search_request = serde_json::json!({
          "query": {
            "type": "terminal",
            "service": "full_text",
            "parameters": {
              "value": search_input
            }
          },
          "return_type": "entry"
        }).to_string();
    dbg!(&search_request);


    let encoded_search_request = encode(&search_request);
    let api_url = format!("https://search.rcsb.org/rcsbsearch/v2/query?json={}", encoded_search_request);
    let client = reqwest::Client::new();

    let search_result_json = client.get(api_url)
        .send()
        .await
        .map_err(|e| {
            dbg!(e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?.text().await;
    // .json::<SearchResult>()
    // .await
    // .map_err(|e| {
    //     dbg!(e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;
    dbg!(&search_result_json);

    Ok(Json(Default::default()))
}