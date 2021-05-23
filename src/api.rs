use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::prelude::*;
use std::str;
use uuid::Uuid;
use hex;

const ENDPOINT: &str = "https://www.paprikaapp.com/api/v2";

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub result: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipesResponse {
    pub result: Vec<RecipeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeEntry {
    pub uid: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub result: Recipe,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipePostResponse {
    pub result: bool,
}

#[derive(Debug, Serialize, Deserialize, Hash, Default, Clone)]
pub struct Recipe {
    pub uid: String,
    pub name: String,
    pub ingredients: String,
    pub directions: String,
    pub description: String,
    pub notes: String,
    pub nutritional_info: String,
    pub servings: String,
    pub difficulty: String,
    pub prep_time: String,
    pub cook_time: String,
    pub total_time: String,
    pub source: String,
    pub source_url: Option<String>,
    pub image_url: Option<String>,
    pub photo: Option<String>,
    pub photo_hash: Option<String>,
    pub photo_large: Option<String>,
    pub scale: Option<String>,
    pub hash: String,
    pub categories: Vec<String>,
    pub rating: i32,
    pub in_trash: bool,
    pub is_pinned: bool,
    pub on_favorites: bool,
    pub on_grocery_list: bool,
    pub created: String,
    pub photo_url: String,
}

impl Recipe {
    // TODO: I don't know if/how Paprika validates hashes. Is this fine?
    fn update_hash(&mut self) {
        let mut hasher = Sha256::new();

        let serialized = serde_json::to_string(&self).unwrap();
        hasher.update(serialized);

        self.hash = hex::encode(hasher.finalize());
    }

    fn generate_uuid(&mut self) {
        self.uid = Uuid::new_v4().to_string();
    }
}

pub async fn login(
    email: &str,
    password: &str,
) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let params = [("email", email), ("password", password)];

    let resp = client
        .get(format!("{}/account/login/", ENDPOINT))
        .form(&params)
        .send()
        .await?;

    let resp_text = resp.text().await?;
    let login_response: Result<LoginResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match login_response {
        Ok(r) => {
            println!("Login response: {:?}", r);
            Ok(r)
        }
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }
}

fn get_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&(String::from("Bearer ") + token)).unwrap(),
    );

    headers
}

pub async fn get_recipes(token: &str) -> Result<RecipesResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/sync/recipes/", ENDPOINT))
        .headers(get_headers(token))
        .send()
        .await?;

    let resp_text = resp.text().await?;
    let login_response: Result<RecipesResponse, serde_json::Error> =
        serde_json::from_str(&resp_text);

    match login_response {
        Ok(r) => {
            println!("Fetched {} recipes", r.result.len());
            Ok(r)
        }
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }
}

pub async fn get_recipe_by_id(token: &str, id: &str) -> Result<Recipe, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/sync/recipe/{}/", ENDPOINT, &recipe.uid))
        .headers(get_headers(token))
        .send()
        .await?;

    let resp_text = resp.text().await?;

    let login_response: Result<RecipeResponse, serde_json::Error> =
        serde_json::from_str(&resp_text);

    match login_response {
        Ok(r) => {
            //println!("Fetched recipe!");
            Ok(r.result)
        }
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }
}

pub async fn upload_recipe(
    token: &str,
    recipe: &mut Recipe,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // new recipes won't have UID
    if recipe.uid == "" {
        recipe.generate_uuid();
    }

    recipe.update_hash();

    // updating/creating recipes seems to have very weird HTTP requirements
    // first, convert to JSON
    let body_json = serde_json::to_vec(&recipe).unwrap();

    // then, GZip-encode that json with no compression
    let mut encoder = GzEncoder::new(Vec::new(), Compression::none());
    encoder.write_all(body_json.as_slice()).unwrap();
    let gzip_body = encoder.finish().unwrap();

    // send that GZip-encoded data as a multi-part file field named "data"
    let part = reqwest::multipart::Part::bytes(gzip_body).file_name("data");
    let form = multipart::Form::new().part("data", part);

    let resp_text = client
        .post(format!("{}/sync/recipe/{}/", ENDPOINT, &recipe.uid))
        .multipart(form)
        .header("accept", "*/*")
        .header("accept-encoding", "utf-8")
        .header("authorization", "Bearer ".to_string() + token)
        .send()
        .await
        .expect("Request failed")
        .text()
        .await
        .expect("Failed to decode response as text");

    println!("TEXT: {:?}", resp_text);

    let login_response: Result<RecipePostResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match login_response {
        Ok(_r) => {
            println!("Updated recipe!");
            Ok(())
        },
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }
}
