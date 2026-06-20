use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum FuelType {
    Espresso,
    ColdBrew,
    Decaf, // never actually used
}

#[derive(Debug)]
struct Spacecraft {
    name: String,
    crew: Vec<String>,
    fuel: FuelType,
    fuel_level: f64,
}

impl Spacecraft {
    fn new(name: &str, fuel: FuelType) -> Self {
        Spacecraft {
            name: name.to_string(),
            crew: Vec::new(),
            fuel,
            fuel_level: 100.0,
        }
    }

    fn board(&mut self, cat: &str) {
        self.crew.push(cat.to_string());
        println!("🐱 {} boarded {}", cat, self.name);
    }

    fn warp(&mut self, distance: f64) -> Result<(), String> {
        let cost = distance * 0.8;
        if self.fuel == FuelType::Decaf {
            return Err("Decaf cannot power a warp drive".to_string());
        }
        if self.fuel_level < cost {
            return Err(format!("Not enough fuel: {:.1} < {:.1}", self.fuel_level, cost));
        }
        self.fuel_level -= cost;
        println!("🚀 Warped {:.1} ly — fuel remaining: {:.1}%", distance, self.fuel_level);
        Ok(())
    }
}

impl fmt::Display for Spacecraft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{:?}] — crew: {}", self.name, self.fuel, self.crew.join(", "))
    }
}

fn main() {
    let mut ship = Spacecraft::new("SS Latte", FuelType::Espresso);
    ship.board("Nebula");
    ship.board("Cosmo");

    match ship.warp(4.2) {
        Ok(_)    => println!("✅ Warp successful"),
        Err(msg) => eprintln!("❌ Warp failed: {}", msg),
    }

    println!("{}", ship);
}
