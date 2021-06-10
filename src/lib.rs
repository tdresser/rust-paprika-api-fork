#[macro_use]
extern crate lazy_static;

pub mod api;

#[cfg(test)]
#[macro_use]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn default_test() {}

    #[tokio::test]
    async fn simple_post_pass() {
        let _m = mock("POST", "/account/login/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .with_body(r#"{"result": {"token": "12345"}}"#)
            .create();

        let res = api::login("user@email.com", "password").await;
        match res {
            Ok(t) => (assert_eq!(t.token, "12345")),
            Err(e) => panic!("Error: {:?}", e),
        };
    }

    #[tokio::test]
    async fn simple_post_empty_body() {
        let _m = mock("POST", "/account/login/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .with_body(r#""#)
            .create();

        let res = api::login("user@email.com", "password").await;
        match res {
            Ok(t) => panic!("Got Ok({:?}, expected Err", t),
            Err(_e) => (),
        };
    }

    #[tokio::test]
    async fn simple_post_no_body() {
        let _m = mock("POST", "/account/login/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .create();

        let res = api::login("user@email.com", "password").await;
        match res {
            Ok(t) => panic!("Got Ok({:?}, expected Err", t),
            Err(_e) => (),
        };
    }

    #[tokio::test]
    async fn simple_get_pass() {
        let _m = mock("GET", "/sync/recipe/12345/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .with_body(r#"{"result":{"uid":"6c4c731e-847d-4e80-a138-125a3a69c5b7","name":"Birria tacos","ingredients":"BIRRIA DE REZ:\n2 pounds boneless chuck\n1 pound oxtail or short ribs\n1 teaspoon neutral oil (avocado or vegetable oil)\nSAUCE:\n7 ancho chiles (ends trimmed and de-seeded)\n7 guajillo chiles (ends trimmed and de-seeded)\n3 chiles de arbol (ends trimmed and de-seeded)\n1 white onion (peeled and halved)\n6 garlic cloves (peeled)\n4 roma tomatoes\n1 tablespoon black peppercorns\n1 teaspoon dried Mexican oregano\n1 teaspoon cumin seeds\n1 teaspoon coriander seeds\n1/4 teaspoon ground cloves\n1/2 Mexican cinnamon stick (See note if not using Mexican cinnamon)\n3 bay leaves\n1 teaspoon apple cider vinegar\n3 cups beef broth or water (divided)\nTACOS:\n1/4 cup minced cilantro\n1/4 white onion (minced)\nJuice from 1 lime\nKosher salt\nCorn tortillas\n3 ounces Oaxacan cheese (or mozzarella)","directions":"TO SEAR THE MEAT:\n\nBring the meat to room temperature, about 30 minutes and then sprinkle liberally on all sides with kosher salt. In a large Dutch oven (or a pot with an oven-proof lid), set over medium-high heat, add the neutral oil. When hot, add the meat and sear on all sides until browned. I like to do a hard sear. You\u2019ll have to do this in batches. Transfer to a bowl.\n\nTO MAKE THE SAUCE:\n\nMeanwhile, in another medium pot, add the dried chiles, halved white onion, garlic cloves, tomatoes, spices, bay leaves and add cold water until it covers everything. Place over medium heat and simmer gently for about 15 minutes. Pour through a strainer and transfer everything (including the whole spices) to a blender. If your blender is small you may need to do this in batches.\n\nAdd the apple cider vinegar and about 1 cup of beef broth or water and blend until very smooth, about 2 minutes. Add salt to taste (I added about 1 tablespoon of kosher salt).\n\nTO BRAISE THE MEAT:\n\nPreheat the oven to 300F. Add the meat back to the pot and pour the sauce over it. To the blender, add the remaining 2 cups of broth or water and swish it around to pick up any leftover sauce and pour it into the pot. Place over medium heat until it reaches a gentle simmer and then immediately cover and transfer to the preheated oven. Cook for about 3 hours, until the meat is tender.\n\nTO ASSEMBLE THE TACOS:\n\nMix together the cilantro, white onion, lime and salt.\n\nRemove the meat from the sauce and shred using two forks. Ladle the broth into a bowl and add a handful of diced cilantro.\n\nAdd a non-stick skillet over medium heat. Dip the tortilla into the top of the broth (this should be fat) and add it to the skillet. Pan fry on one side for about 30 seconds and then flip over. Add a some of the shredded meat and the shredded cheese. Fold over and cook until pan fried on both sides, about 1 minute. Transfer to a plate and serve alongside the broth.","description":"","notes":"*Note: I have a high-powered blender and it resulted in a super smooth sauce. If you have a blender that is meh, you may want to run the sauce through a strainer to discard any big bits the blender didn\u2019t puree. Very optional!\n\nTips and Tricks:\n\nCuts of meat \u2013 You can substitute in goat, lamb or if you can choose oxtail instead of short ribs.\n\nTo make this ahead: I actually think this is a great make-ahead meal. You can make the birria first, shred the meat and store the sauce in the fridge. The fat will solidify on the top. You can use that to the pan with the tortillas if you like. Or you can warm it back up when you\u2019re ready to serve.\n\nMost of these ingredients like Mexican cinnamon, dried chiles and Mexican oregano at a Latin supermarket.\n\nIf you\u2019re not using Mexican cinnamon, remove it and discard it when you\u2019re done boiling all of the chiles. Mexican cinnamon is very brittle and will easily blend up. But if it\u2019s from say Saigon or somewhere else, it tends to be very hard. I wouldn\u2019t put your blender through that!\n\nTo Make it in the Slow-Cooker \u2013 You can make this recipe in the slow-cooker by adding the meat and sauce to a slow cooker. Add the broth and set it to high and let it braise for 6-7 hours.\n\nTo Make it in the Instant Pot \u2013 Sear the meat in the IP. Pour the sauce in, along with the broth. Close the seal, set the setting to \u201Chigh pressure\u201D and press the \u201CStew Meat\u201D option. This should be about 50 minutes. Do a natural release. And it should be perfect!","nutritional_info":"Serving: 1g | Calories: 250kcal (13%) | Carbohydrates: 39g (13%) | Protein: 3g (6%) | Fat: 4g (6%) | Saturated Fat: 2g (13%) | Polyunsaturated Fat: 5g | Cholesterol: 3mg (1%) | Sodium: 46mg (2%) | Fiber: 3g (13%) | Sugar: 1g (1%)","servings":"","difficulty":"","prep_time":"","cook_time":"","total_time":"","source":"Acozykitchen.com","source_url":"https://www.acozykitchen.com/birria-tacos","image_url":null,"photo":null,"photo_hash":null,"photo_large":null,"scale":"","hash":"90F5F353361294068EBF4CCAB4B5415980C96F7CA563F02E0D98FD7565E92ECF","categories":[],"rating":0,"in_trash":true,"is_pinned":false,"on_favorites":false,"on_grocery_list":false,"created":"2021-04-09 15:09:26","photo_url":null}}"#)
            .create();

        let res = api::get_recipe_by_id("token", "12345").await;
        match res {
            Ok(t) => (assert_eq!(t.name, "Birria tacos")),
            Err(e) => panic!("Error: {:?}", e),
        };
    }

    #[tokio::test]
    async fn simple_get_empty_body() {
        let _m = mock("GET", "/sync/recipe/12345/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .with_body(r#""#)
            .create();

        let res = api::get_recipe_by_id("token", "12345").await;
        match res {
            Ok(t) => panic!("Got Ok({:?}, expected Err", t),
            Err(_e) => (),
        };
    }

    #[tokio::test]
    async fn simple_get_no_body() {
        let _m = mock("GET", "/sync/recipe/12345/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .create();

        let res = api::get_recipe_by_id("token", "12345").await;
        match res {
            Ok(t) => panic!("Got Ok({:?}, expected Err", t),
            Err(_e) => (),
        };
    }

    #[tokio::test]
    async fn get_recipe_by_id_invalid_response() {
        let _m = mock("GET", "/sync/recipe/12345/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_header("content-encoding", "gzip")
            .with_body(r#"{"result": {"token": "12345"}}"#)
            .create();

        let res = api::get_recipe_by_id("token", "12345").await;
        match res {
            Ok(t) => panic!("Got Ok({:?}, expected Err", t),
            Err(_e) => (),
        };
    }
}
