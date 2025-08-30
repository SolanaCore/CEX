//TODO!

#[post("/logout")]
async fn logout() {
    
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(logout);
}