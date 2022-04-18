pub struct Allergies {
    allergens: Vec<Allergen>
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
    Unknown = 256
}

impl Allergen {
    pub fn iterator() -> impl DoubleEndedIterator<Item = Allergen> {
        return [Allergen::Eggs, 
                Allergen::Peanuts, 
                Allergen::Shellfish, 
                Allergen::Strawberries,
                Allergen::Tomatoes, 
                Allergen::Chocolate, 
                Allergen::Pollen, 
                Allergen::Cats,
                Allergen::Unknown].iter().copied();
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergens: Vec<Allergen> = vec!();

        let mut current_score = score;
        while current_score > 0 {
            let highest_allergen_available = Allergen::iterator()
            .rev()
            .find(|allergen| { 
                return !allergens.contains(allergen) && (*allergen as u32) <= current_score; 
            });
            if let Some(allergen) = highest_allergen_available {
                if allergen != Allergen::Unknown {
                    allergens.push(allergen);
                }
                current_score -= allergen as u32;
            }
        }
        return Allergies { allergens: allergens };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        return self.allergens.contains(allergen);
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        return self.allergens.to_owned();
    }
}
