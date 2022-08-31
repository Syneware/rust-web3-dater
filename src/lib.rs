use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use web3::types::{Block, BlockId, H256};

pub struct Web3Dater {
    web3client: web3::Web3<web3::transports::Http>,
    blocks_cache: HashMap<u64, Block<H256>>,
}

impl Web3Dater {
    pub fn new(web3client: web3::Web3<web3::transports::Http>) -> Self {
        Self {
            web3client,
            blocks_cache: HashMap::new(),
        }
    }

    pub fn clear_cache(&mut self) {
        self.blocks_cache.clear();
    }

    pub async fn get_block_by_date(&mut self, date: DateTime<FixedOffset>, after: bool) -> Result<Block<H256>, web3::Error> {
        let target_timestamp = date.timestamp();

        let block_number = self.predict_block_by_date(date, None).await?;

        let block_time = self.get_avg_block_time(Some(1000), Some(block_number)).await?;

        let mut predicted_block = self.get_block(block_number).await?;
        let mut diff_time = target_timestamp - predicted_block.timestamp.as_u64() as i64;

        let mut search_block_number = block_number;

        loop {
            if diff_time.abs() < (block_time * 100.0) as i64 {
                break;
            }
            search_block_number = self.predict_block_by_date(date, Some(search_block_number)).await?;
            predicted_block = self.get_block(search_block_number).await?;
            diff_time = target_timestamp - predicted_block.timestamp.as_u64() as i64;
        }

        if diff_time.abs() > (block_time * 5.0) as i64 {
            search_block_number = (search_block_number as i64 + (diff_time as f64 / block_time).round() as i64) as u64;
        }

        loop {
            let block = self.get_block(search_block_number).await?;
            let prev_block = self.get_block(search_block_number - 1).await?;
            let next_block = self.get_block(search_block_number + 1).await?;

            if prev_block.timestamp.as_u64() <= target_timestamp as u64 && target_timestamp as u64 <= next_block.timestamp.as_u64() {
                return if after {
                    if block.timestamp.as_u64() >= target_timestamp as u64 {
                        Ok(block.clone())
                    } else {
                        Ok(next_block.clone())
                    }
                } else if block.timestamp.as_u64() <= target_timestamp as u64 {
                    Ok(block.clone())
                } else {
                    Ok(prev_block.clone())
                };
            }

            if block.timestamp.as_u64() > target_timestamp as u64 {
                search_block_number -= 1;
            } else {
                search_block_number += 1;
            }
        }
    }

    async fn get_block(&mut self, block_number: u64) -> Result<Block<H256>, web3::Error> {
        if self.blocks_cache.contains_key(&block_number) {
            return Ok(self.blocks_cache.get(&block_number).unwrap().clone());
        }
        let new_block = self.web3client.eth().block(BlockId::Number(block_number.into())).await?.unwrap();
        self.blocks_cache.insert(block_number, new_block.clone());
        Ok(new_block)
    }

    async fn predict_block_by_date(&mut self, date: DateTime<FixedOffset>, block_num: Option<u64>) -> Result<u64, web3::Error> {
        let block_time = self.get_avg_block_time(None, block_num).await?;

        let block_number = block_num.unwrap_or(self.web3client.eth().block_number().await?.as_u64());
        let block = self.get_block(block_number).await?;

        let diff_time: i64 = block.timestamp.as_u64() as i64 - date.timestamp();

        Ok((block_number as i64 - (diff_time as f64 / block_time).round() as i64) as u64)
    }

    async fn get_avg_block_time(&mut self, blocks_size: Option<u64>, block_number: Option<u64>) -> Result<f64, web3::Error> {
        let first_last_blocks_size = blocks_size.unwrap_or(100000);

        let latest_block_number = block_number.unwrap_or(self.web3client.eth().block_number().await?.as_u64());

        let first_block = self.get_block(latest_block_number - first_last_blocks_size).await?;
        let last_block = self.get_block(latest_block_number).await?;

        let block_time = ((last_block.timestamp - first_block.timestamp).as_u64() as f64 / first_last_blocks_size as f64) as f64;

        Ok(block_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let transport = web3::transports::Http::new("https://rpc.ankr.com/eth").unwrap();
        let web3client = web3::Web3::new(transport);

        let mut dater = Web3Dater::new(web3client.clone());

        let block = dater
            .get_block_by_date(DateTime::parse_from_rfc3339("2022-08-01T00:00:00+00:00").unwrap(), true)
            .await
            .unwrap();
        println!("{:?}", block);
    }
}
