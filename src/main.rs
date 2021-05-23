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
                Err(e) => Err(e.into())
            }
        } else {
            Err("No password found; is the PAPRIKA_PASSWORD environment variable set?".into())
        }
    } else {
        Err("No email found; is the PAPRIKA_EMAIL environment variable set?".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(token) = login().await {
        println!("Login successfull!");
        let recipe_list = api::get_recipes(&token).await.unwrap().result;
        let recipe = api::get_recipe_by_id(&token, &recipe_list[0].uid).await.unwrap();
        println!("Recipe: {:?}", recipe);

        // print all recipes (can be a lot of requests)
        /*for (_, recipe_entry) in recipe_list.iter().enumerate() {
            let recipe_future = api::get_recipe_by_id(&token, &recipe_entry.uid).await;
            match recipe_future {
                Ok(recipe) => println!("Recipe: {:?}", recipe),
                Err(e) => println!("Error fetching recipe {}: {}", recipe_entry.uid, e)
            }
        }*/

    }
    else {
        return Err("Login failed!".into());
    }
    Ok(())
}