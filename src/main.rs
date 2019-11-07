// use futures::future::try_join_all;
use reqwest::Client;
use serde_json;
use std::{error::Error, time::Instant};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().build()?;
    let now = Instant::now();
    // Get results.
    let mut results = Vec::new();
    // let mut gets = Vec::new();
    for id in 0..20 {
        // let address = format!("{}/{}", base, i);
        // let get = client.get(&address).send();
        let get = get_todo(&client, id);
        // gets.push(get);
        // let result: serde_json::Value = get.await?.json().await?;
        results.push(get.await?);
    }
    // let results = try_join_all(gets).await?;
    // for get in awaited_gets {
    //     let result: serde_json::Value = get.json().await?;
    //     results.push(result);
    // }
    let _ = get_todo(&client, 1);
    println!("Elapsed: {}ms", now.elapsed().as_millis());
    println!("{:#?}", results.last().unwrap());
    Ok(())
}

async fn get_todo(client: &Client, id: i32)
        -> Result<serde_json::Value, Box<dyn Error>> {
    let base = "https://jsonplaceholder.typicode.com/todos";
    let address = format!("{}/{}", base, id);
    Ok(client.get(&address).send().await?.json().await?)
}
