use axum::{Json, Router, routing::get};
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use reqwest;
use reqwest::{Client};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/uniprot", get(uniprot_search));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Default, Serialize, Deserialize)]
struct SearchParams {
    search_term: String,
}

// #[derive(Default, Serialize, Deserialize)]
// struct SearchResults {
//     primaryAccession: String,
// }

async fn uniprot_search(Query(params): Query<SearchParams>) -> Result<Json<String>, StatusCode> {
    let search_term = params.search_term;
    dbg!(&search_term);

    let api_url = format!("https://rest.uniprot.org/uniprotkb/search?query={}", search_term);
    dbg!(&api_url);

    let client: Client = Client::new();
    let search_result_json = client.get(api_url)
        .send()
        .await
        .map_err(|e| {
            dbg!(e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .text()
        .await
        .map_err(|_| StatusCode::CONFLICT)?;


        // .json::<SearchResults>()
        // .await
        // .map_err(|_| StatusCode::CONFLICT)?;


    Ok(Json::from(search_result_json))
}

