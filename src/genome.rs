use rand::Rng;
/// 
/// everything related to genome
/// 
/// 
/// 
const MUTATION_DIVISION: i32 = 10000;// chance value of 1 equals a mutation chance of 0.01%

/// Genome trait
pub trait Genome{
    fn new(e: EatingType) -> Self;
    fn mutate(&mut self, chance: i32);
    fn crossover(&self, other: &Self) -> Self;
    fn to_string(&self) -> String;
    fn get_detection_range(&self) -> f32;
    //fn evaluate_plant(plant: i32);
    //fn compare_strength(&self, other: &Self) -> f32;
    fn increase_energy(&mut self, energy: f32);
    fn has_enough_energy(&self) -> bool;
    fn get_weight(&self) -> f32;
    fn get_power(&self) -> f32;
    fn get_speed(&self) -> f32;
    fn get_eval(&self, num: u8) -> i32;
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
    energy: f32,
    eval_weight_1: i32,
    eval_weight_2: i32,
    eval_weight_3: i32,
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
            energy: 0.0,
            eval_weight_1: rng.gen_range(-1000..1000),
            eval_weight_2: rng.gen_range(-1000..1000),
            eval_weight_3: rng.gen_range(-1000..1000),
        }
    }
    fn mutate(&mut self, chance: i32) {
        //let chance = 5;
        let from = -1.0;
        let to = 1.0;
        let ifrom = -50;
        let ito = 50;
        self.weight = mutate_f32_gene(self.weight, chance, from, to);
        self.speed = mutate_f32_gene(self.speed, chance, from, to);
        self.power = mutate_f32_gene(self.power, chance, from, to);
        self.detection = mutate_f32_gene(self.detection, chance, from, to);
        self.eval_weight_1 = mutate_i32_gene(self.eval_weight_1, chance, ifrom, ito);
        self.eval_weight_2 = mutate_i32_gene(self.eval_weight_2, chance, ifrom, ito);
        self.eval_weight_3 = mutate_i32_gene(self.eval_weight_3, chance, ifrom, ito);
    }

    fn crossover(&self, other: &Self) -> Self {
        BasicGenome {
            etype: self.etype.clone(),
            weight: self.weight,
            speed: other.speed,
            power: self.power,
            detection: other.detection,
            energy: 0.0,
            eval_weight_1: other.eval_weight_1,
            eval_weight_2: self.eval_weight_2,
            eval_weight_3: other.eval_weight_3,
        }
    }

    fn to_string(&self) -> String {
        let name: String;
        match &self.etype {
            EatingType::Herbivore => name = String::from("Herbivore"),
            EatingType::Carnivore => name = String::from("Carnivore"),
            EatingType::Omnivore => name = String::from("Omnivore")
        };
        format!("{}: [w: {}, s: {}, p: {}, d: {}, eval: 1:{} 2:{} 3:{}]",name, self.weight, self.speed, self.power, self.detection, self.eval_weight_1, self.eval_weight_2, self.eval_weight_3)
    }
/*
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
*/
   /* fn compare_strength(&self, other: &Self) -> f32 {
        self.power*self.weight - self.power*self.weight
    }*/

    fn get_detection_range(&self) -> f32 {
        self.detection
    }

    fn increase_energy(&mut self, energy: f32) {
        self.energy += energy;
    }

    fn has_enough_energy(&self) -> bool {
        0.0 <= self.energy - 0.3 * ((0.5 *self.weight) * self.speed) - 0.7 * (self.power * (self.weight * 0.5))
    }

    fn get_power(&self) -> f32 {
        self.power * self.weight
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_eval(&self, num: u8) -> i32 {
        
        match num {
            1 => self.eval_weight_1,
            2 => self.eval_weight_2,
            3 => self.eval_weight_3,
            _ => panic!("wrong eval num")
        }
    }

}
#[derive(Debug, Clone)]
pub enum EatingType {
    Carnivore,
    Herbivore,
    Omnivore
}

fn mutate_f32_gene(gene: f32,chance: i32, from: f32, to: f32) -> f32 {
    if from >= to {
        panic!("Ilegal argument: range to small");
    }
    let mut rng = rand::thread_rng();
    if chance < rng.gen_range(0..MUTATION_DIVISION) {
        return gene;
    }
    gene + rng.gen_range(from..to+1.0)
}

fn mutate_i32_gene(gene: i32,chance: i32, from: i32, to: i32) -> i32 {
    if from >= to {
        panic!("Ilegal argument: range to small");
    }
    let mut rng = rand::thread_rng();
    if chance < rng.gen_range(0..MUTATION_DIVISION) {
        return gene;
    }
    gene + rng.gen_range(from..to+1)
}