pub struct GetUserBalanceRequest {
    pub user_id: String,
}

pub struct GetUserBalanceResponse {
    pub user_id: String,
    pub balances: Vec<Balance>,
}

pub struct Balance {
    pub asset: String,
    pub availabale: f64,
    pub locked_in: f64,
}
