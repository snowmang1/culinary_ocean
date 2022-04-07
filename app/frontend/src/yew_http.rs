// interaction with backend db

// use serde::{Serialize, Deserialize};

pub async fn post_json() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    client.post("localhost:8080/post")
        .body("hello")
        .send()
        .await?;
    Ok(())
}

pub async fn get_post() -> Result<String, reqwest::Error> {
    let req = reqwest::get("localhost:8080/post") //returns reqponse
        .await?
        .text() // gets result<string, error>
        .await?;
    Ok(req)
}
