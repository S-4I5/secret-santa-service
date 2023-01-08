use serde::{Deserialize, Serialize};

//Still in progress

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let body = reqwest::get("http://127.0.0.1:8079/groups")
            .await?
            .json::<Vec<Group>>()
            .await?;

    println!("body = {:?}", body.to_vec());

    Ok(())
}