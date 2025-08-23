use diesel::prelude::*;
use diesel_demo::*;
use crate::models::{User, NewUser, Order, NewOrder, UserBalance, NewUserBalance};
use uuid::Uuid;
use crate::schema::{users, orders, user_balances};
use diesel::result::Error;
use crate::connection::{DB_POOL, DbPool};

pub struct PostgresInvoker {
    pub conn: DbPool,
}

impl PostgresInvoker {
     pub fn new() -> Self {
        Self {
            pool: DB_POOL.clone(),
        }
    }

    pub fn add_user<'a>(&self, new_user: NewUser<'a>) -> Result<User, Error> {
        let mut conn = self.conn.get()?;
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
    }

    pub fn get_user(&self, user_id: Uuid) -> Result<User, Error> {
        let mut conn = self.conn.get()?;
        users::table.filter(users::id.eq(user_id)).first(&mut conn)
    }

    pub fn add_order<'a>(&self, new_order: NewOrder<'a>) -> Result<Order, Error> {
        let mut conn = self.conn.get()?;
        diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result(&mut conn)
    }

    pub fn get_user_orders(&self, user_id: Uuid) -> Result<Vec<Order>, Error> {
        let mut conn = self.conn.get()?;
        orders::table.filter(orders::user_id.eq(user_id)).load(&mut conn)
    }

    pub fn get_user_balance(&self, user_id: Uuid) -> Result<Vec<UserBalance>, Error> {
        let mut conn = self.conn.get()?;
        user_balances::table.filter(user_balances::user_id.eq(user_id)).load(&mut conn)
    }

    pub fn add_user_balance<'a>(
        &self,
        new_user_balance: NewUserBalance<'a>,
    ) -> Result<UserBalance, Error> {
        let mut conn = self.conn.get()?;
        diesel::insert_into(user_balances::table)
            .values(&new_user_balance)
            .get_result(&mut conn)
    }
}
