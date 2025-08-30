//TODO!
#[post("/withdraw")]
async fn withdraw() {
    
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(withdraw);
}