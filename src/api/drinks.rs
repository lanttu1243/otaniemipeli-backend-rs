use deadpool_postgres::Pool;
use rouille::{router, Request, Response};
use crate::utils::runtime::GLOBAL_RT;
use crate::database::{utils::*, drinks::*};
use crate::utils::types::{Drink, DrinkIngredientsNoIngredients, DrinkIngredientsPost, DrinkQty, Ingredient};

pub fn router(pool: Pool) -> impl Fn(&Request) -> Response + Send + Sync + 'static {
    move |request| {
        router!(request,
            (GET) (/api/ingredients) => {
                println!("GET /api/ingredients");
                
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                
                match GLOBAL_RT.block_on(get_ingredients(client)) {
                    Ok(ingredients) if ingredients.ingredients.is_empty() => {
                        Response::text("There are no games").with_status_code(500)},
                    Ok(ingredients) => Response::json(&ingredients).with_status_code(200),
                    Err(_) => Response::text("Error in DB operations").with_status_code(500),
                }
            },
            (POST) (/api/ingredients) => {
                println!("POST /api/ingredients");
                
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                
                let ingredient: Ingredient = match rouille::input::json_input(request) {
                    Ok(v)  => v,
                    Err(_) => return Response::text("Error parsing json-input").with_status_code(500),
                };
                match GLOBAL_RT.block_on(post_ingredient(client, ingredient)) {
                    Err(e) => {
                        eprintln!("{}", e);
                        return Response::text("Error in DB operations").with_status_code(500)
                    }
                    _ => {}};
                Response::text("Ingredient added successfully!").with_status_code(201)
            },
            (GET) (/api/drink) => {
                println!("GET /api/drink");
                
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                
                match GLOBAL_RT.block_on(get_drinks(&client)) {
                    Ok(drinks) if drinks.drinks.is_empty() => Response::text("There are no drinks").with_status_code(500),
                    Ok(drinks) => Response::json(&drinks).with_status_code(200),
                    Err(_) => Response::text("Error in DB operations").with_status_code(500),
                }
            },
            (POST) (/api/drink) => {
                println!("POST /api/drink");
                
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                
                let drink: Drink = match rouille::input::json_input(request) {
                    Ok(v)  => v,
                    Err(_) => return Response::text("JSON parsing error").with_status_code(500),
                };
                match GLOBAL_RT.block_on(create_drink(client, drink)) {
                    Err(e) => {
                        eprintln!("{}", e);
                        return Response::text("Error in DB operations").with_status_code(500)
                    }
                    _ => {}};
                Response::text("Drinks added successfully!").with_status_code(201)
            },
            (GET) (/api/drink/ingredients) => {
                println!("GET /api/drink/ingredients");
                
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                
                match GLOBAL_RT.block_on(get_drinks_ingredients(client)) {
                    Ok(drinks_ingredients) if drinks_ingredients.drink_ingredients.is_empty() => 
                        Response::text("There are no drinks with ingredients").with_status_code(500),
                    Ok(drinks_ingredients) => {
                        match request.get_param("without_ingredients") {
                            Some(str) if str == "true" => {
                                let ret = DrinkQty {
                                    drinks: drinks_ingredients
                                        .drink_ingredients.iter().map(
                                        |dr_ing| DrinkIngredientsNoIngredients {
                                            drink:    dr_ing.drink.clone(),
                                            abv:      dr_ing.abv,
                                            quantity: dr_ing.quantity,
                                        }).collect::<Vec<_>>()
                                };
                                Response::json(&ret).with_status_code(200)
                            },
                            _ => Response::json(&drinks_ingredients).with_status_code(200),
                        }
                    }
                    Err(_) => Response::text("Error in DB operations").with_status_code(500),
                }
                
            },
            (POST) (/api/drink/ingredients) => {
                println!("POST /api/drink/ingredients");
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                let drink_ingredients: DrinkIngredientsPost = match rouille::input::json_input(request) {
                    Ok(v)  => v,
                    Err(_) => return Response::text("JSON parsing error").with_status_code(500),
                };
                match GLOBAL_RT.block_on(add_ingredients(client, drink_ingredients)) {
                    Err(e) => {
                        eprintln!("{}", e);
                        return Response::text("Error in DB operations").with_status_code(500)
                    }
                    _ => {}};
                Response::text("Drink ingredients added successfully!").with_status_code(201)
            },
            (DELETE) (/api/drink/ingredients/{id: i32}) => {
                println!("DELETE /api/drink/ingredients");
                let client = match db_client(&pool) {
                    Ok(c)  => c,
                    Err(r) => return r,
                };
                match GLOBAL_RT.block_on(delete_ingredient(client, id)) {
                    Ok(_) => return Response::text(format!("Ingredient {id} was deleted!")).with_status_code(200),
                    Err(_) => Response::text("Error in DB operations").with_status_code(500),
                }
            },
            _ => Response::empty_404()
        )
    }
}