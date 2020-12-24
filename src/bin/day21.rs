use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn parse(input: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" (");
            let ingredient_list = split.next().unwrap();
            let allergen_list = split
                .next()
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .strip_prefix("contains ")
                .unwrap();
            (
                ingredient_list.split(' ').collect(),
                allergen_list.split(", ").collect(),
            )
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day21/input").unwrap();
    let input = parse(&input);

    // Get a list of all allergens
    let all_allergens: HashSet<&str> = HashSet::from_iter(
        input
            .iter()
            .flat_map(|(_, allergens)| allergens.iter().copied()),
    );
    let all_ingredients: HashSet<&str> = HashSet::from_iter(
        input
            .iter()
            .flat_map(|(ingredients, _)| ingredients.iter().copied()),
    );

    let mut allergens_to_possible_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    for allergen in all_allergens.iter() {
        for (ingredients, allergens) in input.iter() {
            if allergens.contains(allergen) {
                if let Some(translations) = allergens_to_possible_ingredients.get_mut(allergen) {
                    *translations = (*translations)
                        .intersection(&ingredients.iter().copied().collect())
                        .cloned()
                        .collect();
                } else {
                    allergens_to_possible_ingredients
                        .insert(*allergen, ingredients.iter().copied().collect());
                }
            }
        }
    }

    let ingredients_with_allergens: HashSet<&str> = allergens_to_possible_ingredients
        .values()
        .flatten()
        .copied()
        .collect();
    let ingredients_without_allergens: HashSet<&str> = all_ingredients
        .difference(&ingredients_with_allergens)
        .copied()
        .collect();

    let mut count = 0;
    for ingredient in ingredients_without_allergens {
        count += input
            .iter()
            .filter(|(ingredients, _)| ingredients.contains(ingredient))
            .count();
    }

    println!("Solution 1: {}", count);

    let mut allergen_to_ingredient = HashMap::new();
    while let Some((allergen, ingredient)) = allergens_to_possible_ingredients
        .iter()
        .filter_map(|(allergen, ingredients)| {
            if ingredients.len() == 1 {
                Some((*allergen, *ingredients.iter().next().unwrap()))
            } else {
                None
            }
        })
        .next()
    {
        allergen_to_ingredient.insert(allergen, ingredient);
        for (_, ingredients) in allergens_to_possible_ingredients.iter_mut() {
            ingredients.retain(|other_ingredient| *other_ingredient != ingredient);
        }
    }

    let mut allergen_to_ingredient = allergen_to_ingredient.into_iter().collect::<Vec<_>>();
    allergen_to_ingredient.sort_by_key(|(allergen, _)| *allergen);
    let result: String = allergen_to_ingredient
        .iter()
        .map(|(_, ingredient)| *ingredient)
        .intersperse(",")
        .collect();

    println!("Solution 2: {}", result);
}
