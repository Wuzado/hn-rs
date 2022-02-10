use crate::api::{get_beststories, get_item};
use crate::structs;

pub async fn get_n_items(n: u32, func: &dyn FnOnce() -> Result<Vec<i32>, reqwest::Error>) -> Result<Option<Vec<structs::Item>>, Box<dyn std::error::Error>> {
    Ok(Some(items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_1() {
        get_n_items(5, get_beststories());
    }
}