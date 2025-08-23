use crate::connection::REDIS_POOL;
use log::info;
use crate::types::{MessageToSend, MessageFromOrderbook};
use r2d2_redis::redis::{self, Commands};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct MessageWrapper {
    client_id: String,
    message: MessageToSend,
}

pub struct RedisInvoker;

impl RedisInvoker {
    pub fn new() -> Self {
        Self
    }


    /// Publish a message
    pub fn publish_message(&self, channel: &str, message: &str) -> redis::RedisResult<()> {
        let mut conn = REDIS_POOL.get().expect("Failed to get Redis connection");
        info!("Publishing to channel: {}, message: {}", channel, message);
        redis::cmd("PUBLISH").arg(channel).arg(message).execute(&mut *conn);
        Ok(())
    }

    /// Blocking subscribe
    pub fn subscribe(&self, channel: &str) -> redis::RedisResult<()> {
        let mut conn = REDIS_POOL.get().expect("Failed to get Redis connection");
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe(channel)?;

        info!("Subscribed to channel: {}", channel);

        loop {
            let msg = pubsub.get_message()?;
            let payload: String = msg.get_payload()?;
            info!("Received message: {}", payload);
        }
    }

    /// Send a message and wait for response (blocking)
    pub fn send_and_await(&self, message: MessageToSend) -> redis::RedisResult<MessageFromOrderbook> {
        info!("Sending message to engine: {:?}", message);

        let mut sub_conn = REDIS_POOL.get().expect("Failed to get Redis connection");
        let mut pub_conn = REDIS_POOL.get().expect("Failed to get Redis connection");

        //pattern match inside the message to send enum
        let client_id = match &message {
            MessageToSend::PlaceOrder { client_id, .. } => client_id.clone(),
            MessageToSend::CancelOrder { client_id, .. } => client_id.clone(),
            MessageToSend::GetDepth { client_id, .. } => client_id.clone(),
            MessageToSend::GetOpenOrders { client_id, .. } => client_id.clone(),
        };

        info!("Generated client_id = {}", client_id);

        let mut pubsub = sub_conn.as_pubsub();
        pubsub.subscribe(&client_id)?;
        info!("Subscribed to response channel: {}", client_id);

        // Wrap message
        let message_wrapper = MessageWrapper {
            client_id: client_id.clone(),
            message,
        };

        // Push to "messages" queue
        let serialized = serde_json::to_string(&message_wrapper).expect("Failed to serialize message");
        let _: () = pub_conn.lpush("messages", serialized)?;
        info!("Pushed message to Redis list");

        // Wait for response
        let msg = pubsub.get_message()?;
        let payload: String = msg.get_payload()?;
        info!("Received payload = {}", payload);

        pubsub.unsubscribe(&client_id)?;
        info!("Unsubscribed from {}", client_id);

        Ok(serde_json::from_str(&payload).expect("Failed to deserialize response"))
    }
}
