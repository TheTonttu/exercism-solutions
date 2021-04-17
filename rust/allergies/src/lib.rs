use crate::Allergen::*;
use std::slice::Iter;

pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn iter() -> Iter<'static, Allergen> {
        static ALLERGENS: [Allergen; 8] = [
            Cats,
            Pollen,
            Chocolate,
            Tomatoes,
            Strawberries,
            Shellfish,
            Peanuts,
            Eggs,
        ];
        ALLERGENS.iter()
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergens = Allergen::iter()
            .copied()
            .filter(|a| (score & *a as u32) == *a as u32)
            .collect();

        Allergies { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
