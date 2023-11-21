pub struct Allergies {
    score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allergen {
    // Setup for bitwise & comparison
    Eggs = 1,         // xb00000001
    Peanuts = 2,      // xb00000010
    Shellfish = 4,    // xb00000100
    Strawberries = 8, // xb00001000
    Tomatoes = 16,    // xb00010000
    Chocolate = 32,   // xb00100000
    Pollen = 64,      // xb01000000
    Cats = 128,       // xb10000000
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergy_val = *allergen as u32;
        self.score & allergy_val != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergies = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        allergies
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }
}
