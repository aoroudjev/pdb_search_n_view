use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(search_handler()))
        .route("/home", get(home()))
        .route("/hello", get(hello_world()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn hello_world() -> &'static str {
    "Hello, world!"
}

fn home() -> &'static str {
    "Home"
}

fn search_handler() { }


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}