mod api;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
        if let Ok(email) = env::var("PAPRIKA_EMAIL") {
            if let Ok(password) = env::var("PAPRIKA_PASSWORD") {
                let res = api::login(&email, &password).await;
                match res {
                    Ok(t) => println!("Yay! Token: {}", t),
                    Err(e) => panic!("Could not retrieve login token: {}", e)
                }
                //println!("Yay! Token: {}", res);
            }
            else {
                panic!("No password found; is the PAPRIKA_PASSWORD environment variable set?");
            }
        }
        else {
            panic!("No email found; is the PAPRIKA_EMAIL environment variable set?");
        }
        Ok(())
}