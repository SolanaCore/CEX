pub mod connection;
pub use connection::REDIS_POOL;

use r2d2_redis::redis::Commands;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    // Get a connection from the global pool
    let mut conn = REDIS_POOL.get().expect("Failed to get Redis connection from the connection pool");

    // Use the connection
    let _: () = conn.set("DHRUV", "Khankdelwal").unwrap();
    let val: String = conn.get("DHRUV").unwrap();
    println!("Got from Redis: {}", val);
}

