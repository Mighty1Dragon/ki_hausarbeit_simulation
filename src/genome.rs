use rand::Rng;
/// 
/// everything related to genome
/// 
/// 
/// 
const MUTATION_DIVISION: i32 = 10000;// chance value of 1 equals a mutation chance of 0.01%

/// Genome trait
pub trait Genome {
    fn new(e: EatingType) -> Self;
    fn mutate(&mut self, chance: i32);
    fn crossover(&self, other: &Self) -> Self;
    fn to_string(&self) -> String;
    fn get_detection_range(&self) -> f32;
    fn evaluate_creature(&self, other: &Self) -> i32;
    //fn evaluate_plant(plant: i32);
    fn compare_strength(&self, other: &Self) -> f32;
    fn increase_energy(&mut self, energy: f32);
    fn has_enough_energy(&self) -> bool;
    fn get_weight(&self) -> f32;
    fn get_power(&self) -> f32;
    fn get_speed(&self) -> f32;
    //fn choose_direction(())
}

/// basic genome struct
#[derive(Debug, Clone)]
pub struct BasicGenome {
    etype: EatingType,
    weight: f32, //simbolieses the size of the creature
    speed: f32,
    power: f32,
    detection: f32,
    energy: f32
}
//
impl Genome for BasicGenome {
    fn new(etype: EatingType) -> Self {
        let mut rng = rand::thread_rng();
        BasicGenome{
            etype,
            weight: rng.gen_range(0.1..5.0),
            speed: rng.gen_range(0.0..5.0),
            power: rng.gen_range(0.0..5.0),
            detection: rng.gen_range(0.0..5.0),
            energy: 0.0
        }
    }
    fn mutate(&mut self, chance: i32) {
        //let chance = 5;
        let from = -1.0;
        let to = 1.0;
        self.weight = mutate_f32_gene(self.weight, chance, from, to);
        self.speed = mutate_f32_gene(self.speed, chance, from, to);
        self.power = mutate_f32_gene(self.power, chance, from, to);
        self.detection = mutate_f32_gene(self.detection, chance, from, to);
    }

    fn crossover(&self, other: &Self) -> Self {
        BasicGenome {
            etype: self.etype.clone(),
            weight: self.weight,
            speed: other.speed,
            power: self.power,
            detection: other.detection,
            energy: 0.0
        }
    }

    fn to_string(&self) -> String {
        let name: String;
        match &self.etype {
            EatingType::Herbivore => name = String::from("Herbivore"),
            EatingType::Carnivore => name = String::from("Carnivore"),
            EatingType::Omnivore => name = String::from("Omnivore")
        };
        format!("{}: [w: {}, s: {}, p: {}, d: {}]",name, self.weight, self.speed, self.power, self.detection)
    }

    fn evaluate_creature(&self, other: &Self) -> i32 {
        match self.etype {
            EatingType::Carnivore => match other.etype {
                EatingType::Carnivore => 0,
                EatingType::Herbivore => 100,
                EatingType::Omnivore => 100,
            },
            EatingType::Herbivore => match other.etype {
                EatingType::Carnivore => -100,
                EatingType::Herbivore => 0,
                EatingType::Omnivore => -50,
            },
            EatingType::Omnivore => match other.etype {
                EatingType::Carnivore => -100,
                EatingType::Herbivore => 100,
                EatingType::Omnivore => 0,
            }
        }
    }

    fn compare_strength(&self, other: &Self) -> f32 {
        self.power*self.weight - self.power*self.weight
    }

    fn get_detection_range(&self) -> f32 {
        self.detection
    }

    fn increase_energy(&mut self, energy: f32) {
        self.energy += energy;
    }

    fn has_enough_energy(&self) -> bool {
        0.0 <= self.energy - self.weight//(2.0*self.weight)
    }

    fn get_power(&self) -> f32 {
        self.power
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

}
#[derive(Debug, Clone)]
pub enum EatingType {
    Carnivore,
    Herbivore,
    Omnivore
}

fn mutate_f32_gene(mut gene: f32,chance: i32, from: f32, to: f32) -> f32 {
    if from >= to {
        panic!("Ilegal argument: range to small");
    }
    let mut rng = rand::thread_rng();
    if chance < rng.gen_range(0..MUTATION_DIVISION) {
        return gene;
    }
    gene + rng.gen_range(from..to)
}