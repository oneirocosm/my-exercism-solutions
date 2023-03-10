use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

extern crate num as num_renamed;
extern crate num_derive;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, EnumIter)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & num_renamed::ToPrimitive::to_u32(allergen).unwrap() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
