#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat(&mut self, food: impl Food) {
		self.strength += food.gives();
	}
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        4. * self.weight_in_kg
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat_weight = self.weight_in_kg * self.fat_content;
        let protein_weight = self.weight_in_kg * (1.0 - self.fat_content);
        fat_weight * 9.0 + protein_weight * 4.0
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Strength: {}, Score: {}, Money: {}", self.strength, self.score, self.money)?;
        write!(f, "Weapons: {:?}", self.weapons)

    }
}