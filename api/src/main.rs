use actix_web::{App, HttpServer};
use dotenv::dotenv;
use postgres::connection::DB_POOL;
use actix_web::web::Data;
//TODO: CORS

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _db_conn = Data::new(DB_POOL.get());
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}