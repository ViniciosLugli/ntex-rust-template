extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use ntex::{
	http,
	web::{self, middleware, App, HttpRequest, HttpResponse},
};
use ntex_cors::Cors;
use serde_json::json;

async fn index(req: HttpRequest) -> HttpResponse {
	info!("request: {:?}", req);

	HttpResponse::Ok().json(&json!({
		"message": "Hello world!"
	}))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();
	pretty_env_logger::init();

	web::server(|| {
		App::new()
			.wrap(middleware::Logger::default())
			.wrap(
				Cors::new()
					.allowed_origin("*")
					.allowed_methods(vec!["GET"])
					.allowed_headers(vec![http::header::ACCEPT])
					.allowed_header(http::header::CONTENT_TYPE)
					.max_age(3600)
					.finish(),
			)
			.service(web::resource("/").to(index))
	})
	.bind("127.0.0.1:3000")?
	.run()
	.await
}

#[cfg(test)]
mod tests {
	use super::*;
	use ntex::web::{test, App, Error};
	use ntex::{http, web};

	#[ntex::test]
	async fn test_index() -> Result<(), Error> {
		let app = App::new().route("/", web::get().to(index));
		let app = test::init_service(app).await;

		let req = test::TestRequest::get().uri("/").to_request();
		let resp = app.call(req).await.unwrap();

		assert_eq!(resp.status(), http::StatusCode::OK);

		let bytes = test::read_body(resp).await;

		let json = serde_json::from_slice::<serde_json::Value>(&bytes)?;
		assert_eq!(json, json!({ "message": "Hello world!" }));

		Ok(())
	}
}
