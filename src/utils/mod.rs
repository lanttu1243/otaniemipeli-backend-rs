use crate::utils::types::{DrinkIngredients, DrinksIngredients};

pub mod runtime;
pub mod types;
pub mod state;

pub(crate) fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

pub fn remove_ingredients(drinks_ingredients: DrinksIngredients) -> DrinksIngredients {
    DrinksIngredients {
        drink_ingredients: drinks_ingredients
            .drink_ingredients.iter().map(
            |dr_ing| DrinkIngredients {
                drink:    dr_ing.drink.clone(),
                abv:      dr_ing.abv,
                quantity: dr_ing.quantity,
                ingredients: Vec::new(),
            }).collect::<Vec<_>>()
    }
}