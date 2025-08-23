use crate::connection::{REDIS_POOL, RedisPool};
use log::info;
use crate::types::MessageToSend;

pub struct RedisInvoker {
    pub conn: RedisPool,
}

impl RedisInvoker {
    pub fn new() -> Self {
        Self {
            pool: REDIS_POOL.clone(),
        }
    }

    /// Publish a message to a channel
    pub fn publish_message(&self, channel: &str, message: &str) -> redis::RedisResult<()> {
        let mut conn = self.pool.get()?;
        info!("Publishing to channel: {}, message: {}", channel, message);
        conn.publish(channel, message)?;
        Ok(())
    }

    /// Subscribe to a channel
    pub fn subscribe<F>(&self, channel: &str) -> redis::RedisResult<()> {
        let mut conn = self.pool.get()?;
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe(channel)?;

        info!("Subscribed to channel: {}", channel);

        loop {
            let msg = pubsub.get_message()?;
            let payload: String = msg.get_payload()?;

            info!("Received message: {}", payload);
        }
        Ok(())
    }

    /// Placeholder: async sending (future extension)
     /// Async send (using tokio-redis)
    pub async fn send_and_await(&self, message: MessageToSend) -> redis::RedisResult<MessageToSend> {
        let client_id = message.client_id.clone();

        // Get async connection
        let mut conn = self.pool.get()?;
        let mut async_conn = redis::aio::ConnectionLike::from_connection(conn)?;

        // Push message asynchronously
        let serialized = serde_json::to_string(&message).expect("Failed to serialize message");
        let _: () = redis::cmd("LPUSH")
            .arg("messages")
            .arg(&serialized)
            .query_async(&mut async_conn)
            .await?;
        info!("Async pushed message to Redis");

        // Subscribe asynchronously
        let mut pubsub_conn = async_conn.into_pubsub();
        //ASYNC pubsub connection
        pubsub_conn.subscribe(&client_id).await?;
        info!("Subscribed to {}", client_id);

        // Wait for response
        let msg = pubsub_conn.on_message().next().await.unwrap();
        let payload: String = msg.get_payload()?;
        info!("Async received: {:?}", payload);

        pubsub_conn.unsubscribe(&client_id).await?;
        Ok(serde_json::from_str(&payload).expect("Failed to deserialize response"))
    }

}
