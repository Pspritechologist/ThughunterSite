use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let port = std::env::args()
		.find(|arg| arg.starts_with("--port="))
		.and_then(|arg| arg.split_once('=').and_then(|(_, port)| port.parse::<u16>().ok()))
		.unwrap_or_else(|| std::env::var("PORT").ok().and_then(|port| port.parse::<u16>().ok()).unwrap_or(80));

	let app = Router::new()
		.route("/", axum::routing::get(|| async { "Hello, World!" }))
		.route("/reload", axum::routing::get(|| async { "Reloading..." }));

	let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
