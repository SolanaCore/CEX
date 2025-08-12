//https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html

/*
What is a Connection Pool?
A connection pool is like a ready-made set of database connections that your application can reuse instead of opening and closing a new connection every time it needs to talk to the database.
-----
Why use a connection pool?
Opening a new database connection is slow and costly. It takes time and resources.
If every request creates a new connection, it will slow down your app and overload the database.
A connection pool creates a fixed number of connections ahead of time (like a pool of workers).
When your app needs to run a query, it borrows a connection from the pool.
After using it, the connection is returned to the pool for others to reuse.
This makes your app faster and more efficient because it avoids repeatedly connecting/disconnecting.
*/

//Lazy is a helper type that allows you to create a value that is initialized only once, the first time it is accessed — and then reused thereafte
use once_cell::sync::Lazy;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const POOL_CONNECTION_SIZE: u32 = 10;

pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| { 
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(POOL_CONNECTION_SIZE)
        .build(manager)
        .expect("Failed to create connection pool")
});

//How it works under the hood???
  // The pool is initialized exactly once, the first time it’s accessed.
  // It is being reused afterwards