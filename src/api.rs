use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};

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
    pub hash: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub result: Recipe,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe{
    pub uid: String,
    pub name: Option<String>,
    pub ingredients: Option<String>,
    pub directions: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub nutritional_info: Option<String>,
    pub servings: Option<String>,
    pub difficulty: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub image_url: Option<String>,
    pub photo: Option<String>,
    pub photo_hash: Option<String>,
    pub photo_large: Option<String>,
    pub scale: Option<String>,
    pub hash: Option<String>,
    pub categories: Vec<String>,
    pub rating: i32,
    pub in_trash: bool,
    pub is_pinned: bool,
    pub on_favorites: bool,
    pub on_grocery_list: bool,
    pub created: Option<String>,
    pub photo_url: Option<String>,
}

pub async fn login(email: &str, password: &str) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let params = [("email", email), ("password", password)];

    let resp = client
        .post(ENDPOINT.to_owned() + "/account/login/")
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
        .get(ENDPOINT.to_owned() + "/sync/recipes/")
        .headers(get_headers(token))
        .send()
        .await?;

    let resp_text = resp.text().await?;
    let login_response: Result<RecipesResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match login_response {
        Ok(r) => {
            println!("Fetched {} recipes", r.result.len());
            Ok(r)
        },
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }
}

pub async fn get_recipe_by_id(token: &str, id: &str) -> Result<Recipe, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .get(ENDPOINT.to_owned() + "/sync/recipe/" + id + "/")
        .headers(get_headers(token))
        .send()
        .await?;

    let resp_text = resp.text().await?;

    let login_response: Result<RecipeResponse, serde_json::Error> = serde_json::from_str(&resp_text);

    match login_response {
        Ok(r) => {
            println!("Fetched recipe!");
            Ok(r.result)
        },
        Err(e) => {
            println!("Could not deserialize response: {}", resp_text);
            Err(Box::new(e))
        }
    }
}