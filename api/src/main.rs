use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use postgres::connection::DB_POOL;
use actix_web::web::Data;
//TODO: CORS

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_pool = DB_POOL.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                .configure(api::routes::routes_config)
                
            )
            .configure(api::routes::routes_config)
    })
    .bind(())?
    .run()
    .await;
}