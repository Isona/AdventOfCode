use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Mul;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &HashMap<RecipeItem, Vec<RecipeItem>>) -> usize {
    let mut ore_count = 0;
    let mut queue = VecDeque::from([RecipeItem::new(1, "FUEL".to_string())]);
    let mut remainders = HashMap::new();

    while !queue.is_empty() {
        let mut next_output = queue.pop_front().unwrap();
        if let Some(remainder) = remainders.remove(&next_output) {
            if next_output.quantity > remainder {
                next_output.quantity -= remainder;
            } else if next_output.quantity == remainder {
                continue;
            } else {
                let remainder = remainder - next_output.quantity;
                remainders.insert(next_output, remainder);
                continue;
            }
        }

        if let Some((result, ingredients)) = input.get_key_value(&next_output) {
            let times_to_make_recipe = if next_output.quantity < result.quantity {
                remainders.insert(next_output.clone(), result.quantity - &next_output.quantity);
                1
            } else if next_output.quantity == result.quantity {
                1
            } else {
                let times_to_make_recipe = next_output.quantity.div_ceil(result.quantity);
                let remainder = result.quantity * times_to_make_recipe - &next_output.quantity;
                if remainder != 0 {
                    remainders.insert(
                        next_output.clone(),
                        result.quantity * times_to_make_recipe - &next_output.quantity,
                    );
                }
                times_to_make_recipe
            };

            for ingredient in ingredients {
                queue.push_back(ingredient * times_to_make_recipe);
            }
        } else {
            assert_eq!(next_output.material, "ORE");
            ore_count += next_output.quantity
        }
    }

    // We start with 1 fuel
    println!("{remainders:?}");
    ore_count
}

fn part_2(input: &HashMap<RecipeItem, Vec<RecipeItem>>) -> u64 {
    todo!();
}

// 1 DGFK, 1 SXNZP, 1 GCHL => 9 JZWH
fn parse_input(input: &str) -> HashMap<RecipeItem, Vec<RecipeItem>> {
    let mut recipes = HashMap::new();

    for line in input.lines() {
        let mut split = line.trim().split(" => ");
        let input_string = split.next().unwrap();
        let output = RecipeItem::from_string(split.next().unwrap());
        let mut inputs = Vec::new();
        for item in input_string.split(", ") {
            inputs.push(RecipeItem::from_string(item));
        }

        recipes.insert(output, inputs);
    }

    recipes
}

#[derive(Clone, Debug)]
struct RecipeItem {
    quantity: usize,
    material: String,
}

impl RecipeItem {
    fn from_string(input: &str) -> Self {
        let mut split = input.split(" ");
        let quantity = split.next().unwrap().parse().unwrap();
        let material = split.next().unwrap().to_string();

        Self { quantity, material }
    }

    fn new(quantity: usize, material: String) -> Self {
        Self {
            quantity,
            material: material,
        }
    }
}

impl PartialEq for RecipeItem {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material
    }
}

impl Eq for RecipeItem {}

impl Hash for RecipeItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.material.hash(state);
    }
}

impl Mul<usize> for &RecipeItem {
    type Output = RecipeItem;

    fn mul(self, rhs: usize) -> Self::Output {
        RecipeItem::new(self.quantity * rhs, self.material.clone())
    }
}

impl Display for RecipeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} * {}", self.material, self.quantity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
