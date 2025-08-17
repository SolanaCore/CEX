use diesel::prelude::*;
use diesel_demo::*;
use crate::models::{User, NewUser, Order, NewOrder, UserBalance, NewUserBalance};
use uuid::Uuid;
use crate::schema::{users, orders, user_balances};
use diesel::result::Error;

pub struct DatabaseInvoker;

impl DatabaseInvoker {
    // Insert a new user into the users table
    pub fn add_user<'a>(conn: &PgConnection, new_user: NewUser<'a>) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(|e| e)  // Propagate any errors
    }

    // Retrieve a user by user_id
    pub fn get_user(conn: &PgConnection, user_id: Uuid) -> Result<User, Error> {
        users::table
            .filter(users::id.eq(user_id))
            .first(conn)
            .map_err(|e| e)  // Propagate errors, such as user not found
    }

    // Insert a new order into the orders table
    pub fn add_order<'a>(conn: &PgConnection, new_order: NewOrder<'a>) -> Result<Order, Error> {
        diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result(conn)
            .map_err(|e| e)
    }

    // Retrieve all orders for a specific user
    pub fn get_user_orders(conn: &PgConnection, user_id: Uuid) -> Result<Vec<Order>, Error> {
        orders::table
            .filter(orders::user_id.eq(user_id))
            .load::<Order>(conn)
            .map_err(|e| e)
    }

    // Retrieve the user balance
    pub fn get_user_balance(conn: &PgConnection, user_id: Uuid) -> Result<Vec<UserBalance>, Error> {
        user_balances::table
            .filter(user_balances::user_id.eq(user_id))
            .load::<UserBalance>(conn)
            .map_err(|e| e)
    }

    // Insert new user balance data
    pub fn add_user_balance<'a>(
        conn: &PgConnection,
        new_user_balance: NewUserBalance<'a>,
    ) -> Result<UserBalance, Error> {
        diesel::insert_into(user_balances::table)
            .values(&new_user_balance)
            .get_result(conn)
            .map_err(|e| e)
    }
}
