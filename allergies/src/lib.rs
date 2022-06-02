#[derive(Debug, Clone)]
pub struct Allergies { allergens: Vec<Allergen> }


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}



impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { allergens: Allergies::generate_allergens(score) }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }

    fn insert_allergen(allergens: &mut Vec<Allergen>, allergen: Allergen) -> &Vec<Allergen> {
        if !allergens.contains(&allergen) {
          allergens.push(allergen)
        }
        allergens
    }

    fn generate_allergens(mut score: u32) -> Vec<Allergen> {
        let mut allergens: Vec<Allergen> = vec![];
        while score >= 1 {
            match score {
            0 | 1 => {
                let _allergens = &mut allergens;
                score -= 1;
                
                Allergies::insert_allergen(_allergens, Allergen::Eggs);
            },
            2..=3 => {
                let _allergens = &mut allergens;
                score -= 2;
                
                Allergies::insert_allergen(_allergens, Allergen::Peanuts);
            },
            4..=7 => {
                let _allergens = &mut allergens;
                score -= 4;
                
                Allergies::insert_allergen(_allergens, Allergen::Shellfish);
            },
            
            8..=15 => {
                let _allergens = &mut allergens;
                score -= 8;
                
                Allergies::insert_allergen(_allergens, Allergen::Strawberries);
            },
            16..=31 => {
                let _allergens = &mut allergens;
                score -= 16;
                
                Allergies::insert_allergen(_allergens, Allergen::Tomatoes);
            },
            32..=63 => {
                let _allergens = &mut allergens;
                score -= 32;
                
                Allergies::insert_allergen(_allergens, Allergen::Chocolate);
            },
            64..=127 => {
                let _allergens = &mut allergens;
                score -= 64;
                
                Allergies::insert_allergen(_allergens, Allergen::Pollen);
            },
            128.. => {
                let _allergens = &mut allergens;
                score -= 128;
                
                Allergies::insert_allergen(_allergens, Allergen::Cats);
            }
            }
        }
        return allergens;
    }
    
}
