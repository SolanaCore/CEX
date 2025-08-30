use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;
use crate::{
    security::hash_password,
}
use postgres::{
    DB_POOL,
    DbPool as Pool,
    models::NewUser,
    schema::users,
};

//todo: add database connection in the api 
#[post("/signup")]
pub async fn register(
    pool: web::Data<Pool>,
    form: web::Json<SignupRequest>,
) -> impl Responder {
    // Validate fields
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let conn = &mut pool.get().expect("Failed to get DB connection");

    // Hash password
    let hashed_password = hash_password(&form.password);
        .expect("Failed to hash password");
        //TODO: generate a keypair for the user
        //& ALSO ENCRYPT IT WITH THE PASSWORD
    
    let new_user = NewUser {
        username: form.username.clone(),
        email: form.email.clone(),
        password: hashed_password,
        wallet_pubkey,
        wallet_encrypted_privkey,
    };

    // Insert into DB
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Ok().json("User created successfully. Please sign in."),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB Error: {}", e)),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}