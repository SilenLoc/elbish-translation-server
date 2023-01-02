mod jsontransreqhandler;
mod languagefinder;
mod newtranslations;
mod reasonmanagement;
mod routeselb;
mod translateroot;

use crate::routeselb::{new_trans, new_trans_example, ready, trans_example, translate};
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(trans_example)
            .service(translate)
            .service(ready)
            .service(new_trans_example)
            .service(new_trans)
    });

    log::info!("starting HTTP server at http://localhost:8080");

    server.bind(("127.0.0.1", 8080))?.run().await
}
