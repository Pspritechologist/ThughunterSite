use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/", axum::routing::get(|| async { "Hello, World!" }))
		.route("/reload", axum::routing::get(|| async { "Reloading..." }));

	let listener = TcpListener::bind(("0.0.0.0", 80)).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
