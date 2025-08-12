use actix_web::{App, HttpServer};
use dotenv::dotenv;


//TODO: CORS

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(tests)]
pub mod test {

}