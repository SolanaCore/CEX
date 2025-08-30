//TODO!
#[ge("/balance")]
async fn balance() {
    
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(balance);
}