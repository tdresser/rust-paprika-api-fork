use serde::{Deserialize, Serialize};
//use reqwest::Error;
//use std::collections::HashMap;

const ENDPOINT: &str = "https://www.paprikaapp.com/api/v2";
//const ENDPOINT: &str = "https://webhook.site/a3d509f4-c9d5-416f-875c-9884dc9a86b9";

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    result: Token
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

pub async fn login(email: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let params = [("email", email), ("password", password)];

    let resp = client.post(ENDPOINT.to_owned() + "/account/login/").form(&params).send().await?;

    let resp_text = resp.text().await?;

    let login_response: Result<LoginResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match login_response {
        Ok(r) => {
            println!("Login response: {:?}", r);
            Ok(r.result.token.clone())
        },
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }

}