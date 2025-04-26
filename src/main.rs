use axum::{response::Html, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let port = std::env::args()
		.find(|arg| arg.starts_with("--port="))
		.and_then(|arg| arg.split_once('=').and_then(|(_, port)| port.parse::<u16>().ok()))
		.unwrap_or_else(|| std::env::var("PORT").ok().and_then(|port| port.parse::<u16>().ok()).unwrap_or(80));

	let app = Router::new()
		.route("/", axum::routing::get(|| async { template("we thughunting :)", "thughunters") }))
		.route("/death", axum::routing::get(|| async { template("debit", "Sopping wet eggs served daily") }))
		.route("/peng", axum::routing::get(|| async { "benjo" }));

	let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

fn template(title: &str, text: &str) -> Html<String> {
	Html(include_str!("../main_page.html")
		.replace("[[TEXT]]", text)
		.replace("[[TITLE]]", title))
}
