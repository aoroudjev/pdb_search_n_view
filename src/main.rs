use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(hello_world()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn hello_world() -> &'static str{
    "Hello, world!"
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}