A Rust wrapper for the Paprika 3 Recipe Manager API: https://www.paprikaapp.com/

Usage: All requests (except `login`), require a `token` generated from the `login` function.

To update an existing recipe, first fetch it (`get_recipe_by_id`, where the `id` is found from the list of recipes returned by `get_recipes`). Then, edit that recipe and upload it with the same `id`.

To create a new recipe, call `upload_recipe` with a populated `Recipe` that has an empty `uid` field. A new `uid` will be generated for it.
