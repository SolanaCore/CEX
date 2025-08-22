

#[cfg(test)]
pub mod tests {
    use super::*;
    #[tokio::test]
    async fn insert_test_data(pool_manager: &Arc<PoolManager>) -> Result<(), Box<dyn Error>> {
    println!("Inserting test data");
    
    let mut prices = Vec::new();
    for i in 0..100 {
        for asset in ["SOL", "BTC"] {
            prices.push(Price::new(
                Uuid::new_v4(),
                asset.to_string(),
                42.58 + (i as f64 * 0.01),
            ));
        }
    }

    if prices.is_empty() {
        return Ok(());
    }

    for chunk in prices.chunks(20) {
        let blob = chunk.to_vec();

        let query = Price::batch_insert_query(blob.as_slice());

        pool_manager.execute_with_retry(&query)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to insert price: {}", e))?;
    }

    println!("Inserted {} test prices", prices.len());
    
    Ok(())
}
}