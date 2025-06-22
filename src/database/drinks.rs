use deadpool_postgres::Client;
use crate::utils::{
    round, 
   types::{
        Drink,
        DrinkIngredients,
        Drinks,
        DrinksIngredients,
        Ingredient,
        IngredientQty,
        Ingredients,
        PgError
   }
};
use crate::utils::types::DrinkIngredientsPost;

pub async fn get_ingredients(client: Client) -> Result<Ingredients, PgError>{
    
    let query_str = "\
    SELECT ingredient_id, name, abv, carbonated FROM ingredients";
    
    let mut ingredients: Vec<Ingredient> = Vec::new();
    
    let query = match client.query(query_str, &[]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    for row in query {
        let ingredient: Ingredient = Ingredient {
            id: row.get(0),
            name: row.get(1),
            abv: row.get(2),
            carbonated: row.get(3),
        };
        ingredients.push(ingredient);
    }
    Ok(Ingredients{ingredients})
}
pub async fn post_ingredient(client: Client, ingredient: Ingredient) -> Result<u64, PgError> {
    let query_str = "\
    INSERT INTO ingredients (name, abv, carbonated) VALUES ($1, $2, $3)";
    
    client.execute(query_str, &[
        &ingredient.name, 
        &ingredient.abv, 
        &ingredient.carbonated
    ]).await
}
pub async fn delete_ingredient(client: Client, ingredient_id: i32) -> Result<u64, PgError> {
    let query_str = "\
    DELETE FROM ingredients WHERE ingredient_id = $1";
    
    client.execute(query_str, &[&ingredient_id]).await
}
pub async fn create_drink(client: Client, drink: Drink) -> Result<u64, PgError> {
    let query_str = "\
    INSERT INTO drinks (name) VALUES ($1)";
    
    client.execute(query_str, &[&drink.name]).await
}

pub async fn get_drinks(client: &Client) -> Result<Drinks, PgError> {
    let query_str = "\
    SELECT drink_id, name FROM drinks";
    
    let mut drinks: Vec<Drink> = Vec::new();
    
    let client = match client.query(query_str, &[]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    for row in client {
        let drink: Drink = Drink {
            id: row.get(0),
            name: row.get(1),
        };
        drinks.push(drink);
    }
    Ok(Drinks {drinks})
}
pub async fn add_ingredient(client: &Client, drink_id: i32, ingredient_id: i32, quantity: f64) -> Result<u64, PgError> {
    let query_str = "\
    INSERT INTO drink_ingredients (drink_id, ingredient_id, quantity) VALUES ($1, $2, $3)";
    
    client.execute(query_str, &[&drink_id, &ingredient_id, &quantity]).await
}
pub async fn add_ingredients(client: Client, drink_ingredient: DrinkIngredientsPost) -> Result<u64, PgError> {
    let drink_id: i32 = drink_ingredient.drink.id;
    let mut rows = 0;
    for ingredient in &drink_ingredient.ingredients {
        match add_ingredient(&client, drink_id, ingredient.ingredient.id, ingredient.quantity).await {
            Ok(_) => rows += 1,
            Err(e) => return Err(e)
        }
    }
    Ok(rows)
}

pub async fn get_drink_ingredients(client: &Client, drink: Drink) -> Result<DrinkIngredients, PgError> {
    let query_str = "\
    SELECT \
        d.drink_id, \
        d.ingredient_id, \
        i.name, \
        i.abv, \
        i.carbonated, \
        d.quantity \
    FROM drink_ingredients as d \
    INNER JOIN ingredients as i on d.ingredient_id = i.ingredient_id \
    WHERE d.drink_id = $1 \
    ORDER BY d.ingredient_id";
    
    let mut drink_ingredients: DrinkIngredients = DrinkIngredients {
        drink,
        abv: 0.0,
        quantity: 0.0,
        ingredients: Vec::new(),
    };
    
    let query = match client.query(query_str, &[&drink_ingredients.drink.id]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    for row in query {
        let ingredient: Ingredient = Ingredient {
            id: row.get(1),
            name: row.get(2),
            abv: row.get(3),
            carbonated: row.get(4),
        };
        let ingredient_qty = IngredientQty {
            ingredient,
            quantity: row.get(5),
        };
        drink_ingredients.ingredients.push(ingredient_qty);
    }
    Ok(drink_ingredients)
}

pub async fn get_drinks_ingredients(client: Client) -> Result<DrinksIngredients, PgError> {
    let mut drink_ingredients: Vec<DrinkIngredients> = Vec::new();
    let drinks = match get_drinks(&client).await {
        Ok(drinks) if drinks.drinks.is_empty() => {
            return Ok(DrinksIngredients { drink_ingredients }) 
        },
        Ok(drinks) => drinks,
        Err(e) => return Err(e)
    };
    
    for drink in drinks.drinks {
        let mut drinks_ingredient: DrinkIngredients = match get_drink_ingredients(&client, drink).await {
            Ok(di) if di.ingredients.is_empty() => return Ok(DrinksIngredients {drink_ingredients}),
            Ok(di) => di,
            Err(e) => return Err(e)
        };
        let ingr = drinks_ingredient
            .ingredients.iter();
        let qty = ingr.clone().fold(0.0, |acc, iq| acc + iq.quantity);
        let abv = ingr.clone().fold(0.0, |acc, iq| acc + iq.ingredient.abv * iq.quantity);
        drinks_ingredient.abv = round(abv / qty, 1);
        drinks_ingredient.quantity = round(qty, 2);
        drink_ingredients.push(drinks_ingredient);
    }
    Ok(DrinksIngredients {drink_ingredients})
}