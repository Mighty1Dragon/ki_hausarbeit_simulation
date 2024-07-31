use rand::Rng;
/// 
/// everything related to genome
/// 
/// 
/// 
const MUTATION_DIVISION: i32 = 1000;// chance value of 1 equals a mutation chance of 0.01%

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
        let max1 = 5.0;
        let max2 = 1000;
        let min = -1 * max2;
        
        BasicGenome{
            etype,
            weight: rng.gen_range(0.1..max1),
            speed: rng.gen_range(0.0..max1),
            power: rng.gen_range(0.0..max1),
            detection: rng.gen_range(0.0..max1),
            energy: 0.0,
            eval_weight_1: rng.gen_range(min..max2),
            eval_weight_2: rng.gen_range(min..max2),
            eval_weight_3: rng.gen_range(min..max2),
        }
    }
    fn mutate(&mut self, chance: i32) {
        //let chance = 5;
        let mut rng = rand::thread_rng();
        if chance < rng.gen_range(0..MUTATION_DIVISION) {
            return;
        }
        let from = -1.0;
        let to = 1.0;
        let ifrom = -50;
        let ito = 50;
        let choosen = rng.gen_range(0..7);
        match choosen {
            0 => self.weight = mutate_f32_gene(self.weight, from, to),
            1 => self.speed = mutate_f32_gene(self.speed, from, to),
            2 => self.power = mutate_f32_gene(self.power, from, to),
            3 => self.detection = mutate_f32_gene(self.detection, from, to),
            4 => self.eval_weight_1 = mutate_i32_gene(self.eval_weight_1, ifrom, ito),
            5 => self.eval_weight_2 = mutate_i32_gene(self.eval_weight_2, ifrom, ito),
            6 => self.eval_weight_3 = mutate_i32_gene(self.eval_weight_3, ifrom, ito),
            _ => panic!("choosen gene does not exist")
        }
        if self.weight < 0.1 {
            self.weight = 0.1;
        }
        if self.power < 0.0 {
            self.power = 0.0;
        }
        if self.speed < 0.0 {
            self.speed = 0.0;
        }
        
        if self.detection < 0.0 {
            self.detection = 0.0;
        }
        
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
        0.0 <= self.energy - 0.2 *(self.weight + self.power + self.detection + self.speed)
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
#[allow(warnings)]
pub enum EatingType {
    Carnivore,
    Herbivore,
    Omnivore
}

fn mutate_f32_gene(gene: f32, from: f32, to: f32) -> f32 {
    if from >= to {
        panic!("Ilegal argument: range to small");
    }
    let mut rng = rand::thread_rng();
    gene + rng.gen_range(from..to+1.0)
}

fn mutate_i32_gene(gene: i32, from: i32, to: i32) -> i32 {
    if from >= to {
        panic!("Ilegal argument: range to small");
    }
    let mut rng = rand::thread_rng();
    gene + rng.gen_range(from..to+1)
}