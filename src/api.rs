use flate2::write::GzEncoder;
use flate2::Compression;
use hex;
#[cfg(test)]
use mockito;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::prelude::*;
use std::str;
use uuid::Uuid;

#[cfg(not(test))]
lazy_static! {
    static ref URL: String = String::from("https://www.paprikaapp.com/api/v2");
}

#[cfg(test)]
lazy_static! {
    static ref URL: String = mockito::server_url();
}

pub enum QueryType {
    GET,
    POST,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: ApiResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)] // this is what lets serde guess at how to deserialize ApiResponse properly
pub enum ApiResult {
    Token(Token),
    Bool(bool),
    Recipes(Vec<RecipeEntry>),
    Categories(Vec<Category>),
    Recipe(Recipe),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeEntry {
    pub uid: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub uid: String,
    pub order_flag: i32,
    pub name: String,
    pub parent_uid: Option<String>,
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
    pub photo_url: Option<String>,
}

impl Recipe {
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

fn get_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&(String::from("Bearer ") + token)).unwrap(),
    );

    headers
}

pub async fn simple_query(
    token: &str,
    endpoint: &str,
    query_type: QueryType,
    form_args: Option<Box<[(&str, &str)]>>,
//) -> Result<ApiResult, serde_json::Error> {
) -> Result<ApiResult, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut builder: reqwest::RequestBuilder;

    match query_type {
        QueryType::GET => builder = client.get(format!("{}/{}/", &**URL, endpoint)),
        QueryType::POST => builder = client.post(format!("{}/{}/", &**URL, endpoint)),
    }

    if let Some(t) = form_args {
        builder = builder.form(&*t);
    }

    let resp_text = builder
        .headers(get_headers(token))
        .send()
        .await?
        .text()
        .await?;

    let response: Result<ApiResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match response {
        Ok(r) => Ok(r.result),
        Err(e) => Err(e.into()),
    }
}

pub async fn login(email: &str, password: &str) -> Result<Token, Box<dyn std::error::Error>> {
    let params = [("email", email), ("password", password)];

    let token = simple_query("", "account/login", QueryType::POST, Some(Box::new(params))).await;

    match token {
        Ok(r) => match r {
            ApiResult::Token(r) => Ok(r),
            _ => Err("Invalid API response".into()),
        },
        Err(e) => Err(e),
    }
}

pub async fn get_recipes(token: &str) -> Result<Vec<RecipeEntry>, Box<dyn std::error::Error>> {
    let recipes = simple_query(token, "sync/recipes", QueryType::GET, None).await;

    match recipes {
        Ok(r) => match r {
            ApiResult::Recipes(r) => Ok(r),
            _ => Err("Invalid API response".into()),
        },
        Err(e) => Err(e),
    }
}

pub async fn get_categories(token: &str) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
    let categories = simple_query(token, "sync/categories", QueryType::GET, None).await;

    match categories {
        Ok(r) => match r {
            ApiResult::Categories(r) => Ok(r),
            _ => Err("Invalid API response".into()),
        },
        Err(e) => Err(e),
    }
}

pub async fn get_recipe_by_id(token: &str, id: &str) -> Result<Recipe, Box<dyn std::error::Error>> {
    let endpoint = format!("{}/{}", "sync/recipe", &id);
    let recipe = simple_query(token, &&endpoint, QueryType::GET, None).await;

    match recipe {
        Ok(r) => match r {
            ApiResult::Recipe(r) => Ok(r),
            _ => Err("Invalid API response".into()),
        },
        Err(e) => Err(e),
    }
}

/// Uploads a new recipe (when uid == "") or updates an existing one (when uid exists in database already)
/// #arguments
/// * `token` a login token from `login()`
/// * `recipe` a populated Recipe, with or without a `uid` (for updating or creating a new recipe)
pub async fn upload_recipe(
    token: &str,
    recipe: &mut Recipe,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // new recipes won't have UID
    if recipe.uid.is_empty() {
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
        .post(format!("{}/sync/recipe/{}/", &**URL, &recipe.uid))
        .multipart(form)
        .header("accept", "*/*")
        .header("accept-encoding", "utf-8")
        .header("authorization", "Bearer ".to_string() + token)
        .send()
        .await?
        .text()
        .await?;

    let recipe_post_resp: Result<ApiResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match recipe_post_resp {
        Ok(r) => match r.result {
            ApiResult::Bool(b) => Ok(b),
            _ => Err("Recipe POST failed".into()),
        },
        Err(e) => Err(Box::new(e)),
    }
}
