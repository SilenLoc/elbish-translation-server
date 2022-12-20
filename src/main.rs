mod routeselb;

use crate::routeselb::{example, ready, translate};
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(example)
            .service(translate)
            .service(ready)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
