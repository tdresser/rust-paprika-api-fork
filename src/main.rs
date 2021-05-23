pub mod api;

use std::env;

async fn login() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(email) = env::var("PAPRIKA_EMAIL") {
        if let Ok(password) = env::var("PAPRIKA_PASSWORD") {
            let res = api::login(&email, &password).await;
            match res {
                Ok(t) => {
                    println!("Yay! Token: {}", t.result.token);
                    Ok(t.result.token)
                }
                Err(e) => Err(e.into()),
            }
        } else {
            Err("No password found; is the PAPRIKA_PASSWORD environment variable set?".into())
        }
    } else {
        Err("No email found; is the PAPRIKA_EMAIL environment variable set?".into())
    }
}

// print all recipes (can be a lot of requests)
#[allow(dead_code)]
async fn list_recipes(token: &str) {
    let recipe_list = api::get_recipes(&token).await.unwrap().result;
    for (_, recipe_entry) in recipe_list.iter().enumerate() {
        let recipe_future = api::get_recipe_by_id(&token, &recipe_entry.uid).await;
        match recipe_future {
            Ok(recipe) => println!("Recipe: {:?}", recipe),
            Err(e) => println!("Error fetching recipe {}: {}", recipe_entry.uid, e),
        }
    }
}

#[allow(dead_code)]
async fn update_recipe(token: &str) {
    let mut recipe = api::get_recipe_by_id(&token, "FD9A4450-8768-41E5-9121-3658A7411AB0")
        .await
        .unwrap();

    recipe.name = String::from("Birria tacos");
    api::upload_recipe(&token, &mut recipe).await.unwrap();

    let recipe_after_edit = api::get_recipe_by_id(&token, &recipe.uid).await.unwrap();

    println!("Edited recipe: \n{:?}", recipe_after_edit);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(_token) = login().await {
        println!("Login successful!");
    } else {
        return Err("Login failed!".into());
    }
    Ok(())
}
