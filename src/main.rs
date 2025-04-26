use axum::{response::Html, Router};
use base64::Engine;
use tokio::net::TcpListener;

static IMAGES: [&[u8]; 9] = [
	include_bytes!("../ralsei/01.png"),
	include_bytes!("../ralsei/02.png"),
	include_bytes!("../ralsei/03.png"),
	include_bytes!("../ralsei/04.png"),
	include_bytes!("../ralsei/05.png"),
	include_bytes!("../ralsei/06.png"),
	include_bytes!("../ralsei/07.png"),
	include_bytes!("../ralsei/08.png"),
	include_bytes!("../ralsei/09.png"),
];

#[tokio::main]
async fn main() {
	let port = std::env::args()
		.find(|arg| arg.starts_with("--port="))
		.and_then(|arg| arg.split_once('=').and_then(|(_, port)| port.parse::<u16>().ok()))
		.unwrap_or_else(|| std::env::var("PORT").ok().and_then(|port| port.parse::<u16>().ok()).unwrap_or(80));

	let app = Router::new()
		.route("/", axum::routing::get(|| async { image(":ralseiblunt:", fastrand::choice(IMAGES).unwrap()) }))
		.route("/park", axum::routing::get(|| async { template_text("we thughunting :)", "thughunters") }))
		.route("/death", axum::routing::get(|| async { template_text("debit", "Sopping wet eggs served daily") }))
		.route("/peng", axum::routing::get(|| async { "benjo" }));

	let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

fn image(title: &str, image: &[u8]) -> Html<String> {
	let encoded_image = base64::prelude::BASE64_STANDARD.encode(image);
	template(title, &format!("<img src='data:image/png;base64,{encoded_image}' alt='Horray Image'>"))
}

fn template_text(title: &str, text: &str) -> Html<String> {
	template(title, &format!("<h1>{text}</h1>"))
}

fn template(title: &str, body: &str) -> Html<String> {
	Html(include_str!("../main_page.html")
		.replace("[[BODY]]", body)
		.replace("[[TITLE]]", title))
}
