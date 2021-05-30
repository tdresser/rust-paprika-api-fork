# Paprika API

A Rust wrapper for the Paprika 3 Recipe Manager API: https://www.paprikaapp.com/

# Usage
**Include the library:**  
```rust
use paprika_api::api;
```

**Generate a login token:**
```rust
// Logs in via environment variables, but you can choose whatever method you like.
// The scope starting at `let res = api::login(&email, &password).await;` is what really matters
async fn login() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(email) = env::var("PAPRIKA_EMAIL") {
        if let Ok(password) = env::var("PAPRIKA_PASSWORD") {
            let res = api::login(&email, &password).await;
            match res {
                Ok(t) => {
                    println!("Yay! Token: {}", t.token);
                    Ok(t.token)
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
```
`paprika-api` does not cache the login token. Applications using `paprika-api` must cache the token themselves.

**Fetch and print all recipes:**  
```rust
async fn list_recipes(token: &str) {
    let recipe_list = api::get_recipes(&token).await.unwrap();

    for (_, recipe_entry) in recipe_list.iter().enumerate() {
        let recipe_future = api::get_recipe_by_id(&token, &recipe_entry.uid).await;
        match recipe_future {
            Ok(recipe) => println!("Recipe: {:?}", recipe),
            Err(e) => println!("Error fetching recipe {}: {}", recipe_entry.uid, e),
        }
    }
}
```

**Update existing recipe:**
```rust
async fn update_recipe(token: &str, id: &str) {
    let mut recipe = api::get_recipe_by_id(&token, &id)
        .await
        .unwrap();

    recipe.name = String::from("Updated recipe");
    let success = api::upload_recipe(&token, &mut recipe).await.unwrap();

    if success {
        let recipe_after_edit = api::get_recipe_by_id(&token, &recipe.uid).await.unwrap();
        println!("Edited recipe: \n{:?}", recipe_after_edit);
    } else {
        println!("Failed to update recipe");
    }
}
```

**Create new recipe:**  
*Important note:* Paprika does some field validation. If the recipe creation fails, it's likely that you have a field with invalid data. The following is a working example
```rust
async fn create_recipe(token: &str) {
    let mut recipe = api::Recipe {
        uid: "".into(),
        name: "Birria tacos".into(),
        ingredients: "None!".into(),
        directions: "None!".into(),
        description: "None!".into(),
        notes: "".into(),
        nutritional_info: "".into(),
        servings: "".into(),
        difficulty: "".into(),
        prep_time: "".into(),
        cook_time: "".into(),
        total_time: "".into(),
        source: "acozykitchen.com".into(),
        source_url: Some("https://www.acozykitchen.com/birria-tacos".into()),
        image_url: Some("https://www.acozykitchen.com/wp-content/uploads/2021/01/BirriaTacos-11-1227x1536-2-500x500.jpg".into()),
        photo: Some("CB5F52D6-74FF-499D-8793-5FFC8190C6DC.jpg".into()),
        photo_hash: Some("36E72B4585E7ECD10AC6EF5B331789E7004BDB1F9607BC22BE27759CDD143FB6".into()),
        photo_large: None,
        scale: None,
        hash: "".into(),
        categories: vec!(),
        rating: 1,
        in_trash: false,
        is_pinned: false,
        on_favorites: false,
        on_grocery_list: false,
        created: "2021-04-09 15:09:26".into(),
        photo_url: Some("photo".into()),
    };

    recipe.uid = "".into();

    let success = api::upload_recipe(&token, &mut recipe).await.unwrap();

    if success {
        // `upload_recipe` generates a UID for us
        let recipe_after_upload = api::get_recipe_by_id(&token, &recipe.uid).await.unwrap();
        println!("New recipe: \n{:?}", recipe_after_upload);
    } else {
        println!("Failed to create recipe");
    }
}
```

Usage: All requests (except `login`), require a `token` generated from the `login` function.

To update an existing recipe, first fetch it (`get_recipe_by_id`, where the `id` is found from the list of recipes returned by `get_recipes`). Then, edit that recipe and upload it with the same `id`.

To create a new recipe, call `upload_recipe` with a populated `Recipe` that has an empty `uid` field. A new `uid` will be generated for it.
