use std::{
    future::Future,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{de::DeserializeOwned, Serialize};

const CACHE_THRESHOLD: u64 = 60 * 60;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Cache<T> {
    pub fetch_time: i64,
    pub data: T,
}

/// returns the current timestamp in seconds
fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub async fn fetch_with_cache<T, FetchFn, Fut>(
    fetch_type: &str,
    league: &str,
    fetch_fn: FetchFn,
) -> std::io::Result<T>
where
    T: DeserializeOwned + Serialize + Clone,
    FetchFn: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    let fetch_time = timestamp();

    let fname = format!("/tmp/__poe__{}__{}.json", fetch_type, league);
    let cache_path = PathBuf::from(fname);

    // use cache if it is available
    if cache_path.exists() {
        let cache = std::fs::read_to_string(&cache_path)?;
        let cache = serde_json::from_str::<Cache<T>>(&cache)?;

        // use cache if it is not older than 1 hour
        if fetch_time - (cache.fetch_time as u64) < CACHE_THRESHOLD {
            return Ok(cache.data);
        }
    }

    let data = fetch_fn().await;
    // cache not available or outdated, fetch data
    let cache = Cache {
        fetch_time: fetch_time as i64,
        data: data.clone(),
    };
    let cache = serde_json::to_string(&cache)?;
    std::fs::write(&cache_path, cache)?;

    Ok(data)
}
