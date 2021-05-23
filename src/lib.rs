mod api;

use std::env;

#[cfg(test)]
#[macro_use]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
      }

    #[test]
    fn login() {
        if let Ok(email) = env::var("PAPRIKA_EMAIL") {
            if let Ok(password) = env::var("PAPRIKA_PASSWORD") {
                let res = aw!(api::login(&email, &password));
                match res {
                    Ok(t) => println!("Yay! Token: {}", t),
                    Err(e) => panic!("Could not retrieve login token: {}", e)
                }
            }
            else {
                panic!("No password found; is the PAPRIKA_PASSWORD environment variable set?");
            }
        }
        else {
            panic!("No email found; is the PAPRIKA_EMAIL environment variable set?");
        };
    }
}