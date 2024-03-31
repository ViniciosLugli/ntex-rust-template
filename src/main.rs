extern crate pretty_env_logger;
#[macro_use] extern crate log;

use ntex::web::{self, middleware, App, HttpRequest};

async fn index(req: HttpRequest) -> &'static str {
	info!("request: {:?}", req);
	"Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

	web::server(|| App::new().wrap(middleware::Logger::default()).service(web::resource("/").to(index)))
		.bind("127.0.0.1:3000")?
		.run()
		.await
}

#[cfg(test)]
mod tests {
	use super::*;
	use ntex::util::Bytes;
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

		assert_eq!(bytes, Bytes::from(r##"Hello world!"##));

		Ok(())
	}
}
