use std::{collections::{btree_map::Keys, HashMap}, rc::Rc, thread::Thread};

use genome::{BasicGenome, Genome};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::rngs::mock::StepRng;

mod genome;
mod ui;

const PLANT_ENERGY: f32 = 1.0;

trait Simulation{
    fn new(epochs: u16, sim_time: u16, mutation_chance: i32) -> Self;
    fn run(&mut self);
}

struct BasicSimulation <T : Genome, E : Genome>{
    epochs: u16,
    sim_time: u16,
    mutation_chance: i32,
    plants: HashMap<(i32,i32), bool>,
    herbi: HashMap<(i32, i32), T>,
    carni: HashMap<(i32, i32), E>,
}

impl<T : genome::Genome,E : genome::Genome> Simulation for BasicSimulation<T, E>{
    fn new(epochs: u16, sim_time: u16, mutation_chance: i32) -> Self {
        let mut plants:HashMap<(i32,i32), bool> = HashMap::new();
        let mut herbi:HashMap<(i32,i32), T> = HashMap::new();
        let mut carni:HashMap<(i32,i32), E> = HashMap::new();
        //placing plants for food
        for _ in 0..300 {
            let k = gen_pos();
            if !plants.contains_key(&k){
                plants.insert(k, true);
            }
        };
        //placing herbivores
        for _ in 0..100{
            let k = gen_pos();
            if !herbi.contains_key(&k){
                herbi.insert(k, T::new(genome::EatingType::Herbivore));
            }
        };
        //placing carnivoress
        for _ in 0..100{
            let k = gen_pos();
            if !carni.contains_key(&k){
                carni.insert(k, E::new(genome::EatingType::Carnivore));
            }
        };
        BasicSimulation {
            epochs, sim_time, mutation_chance,
            plants, herbi, carni
        }
    }

    fn run(&mut self) {
        for e in 0..self.epochs{
            //Epoch Output
            let mut herbi_keys: Vec<(i32,i32)> = self.herbi.keys().cloned().collect();
            let mut carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
            println!("{}. epoch genes:", e+1);
            for g in herbi_keys{
                println!("{}",self.herbi.get(&g).expect("herbi not available").to_string());
            }
            for g in carni_keys{
                println!("{}",self.carni.get(&g).expect("herbi not available").to_string());
            }
            for s in 0..self.sim_time{
                let mut rng = thread_rng();
                let mut herbi_keys: Vec<(i32,i32)> = self.herbi.keys().cloned().collect();
                let mut carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
                herbi_keys.shuffle( &mut rng);
                carni_keys.shuffle( &mut rng);
                //Herbi move
                for h in herbi_keys{
                    if self.carni.contains_key(&h) {
                        let dead = self.herbi.remove(&h);
                        self.carni.get_mut(&h)
                        .expect("carni not existend")
                        .increase_energy(
                            dead.expect("herbi not existend").get_weight()
                        );
                        continue;
                    }
                    //choosing direction
                    let herbi_direction = herbi_detect(h, &self.carni, &self.herbi, &self.plants);
                    let temp = self.herbi.remove(&h).expect("herbi does not exist");
                    //moving a step
                    let new_pos = add_2x_tupel(herbi_direction, h);
                    self.herbi.insert(new_pos, temp);

                    if self.plants.contains_key(&new_pos){
                        self.plants.remove_entry(&new_pos);
                        self.herbi.get_mut(&new_pos).expect("herbi does not exist").increase_energy(PLANT_ENERGY);
                    }
                }
                //Carni Move
                for c in carni_keys {
                    let carni_direction = carni_detect(c, &self.carni, &self.herbi);
                    let temp = self.carni.remove(&c).expect("no carni :(");
                    let new_pos = add_2x_tupel(carni_direction, c);
                    self.carni.insert(new_pos, temp);
                    if self.herbi.contains_key(&new_pos) {
                        let dead = self.herbi.remove(&new_pos).expect("no herbi :(");
                        self.carni.get_mut(&new_pos).expect("no carni :(").increase_energy(dead.get_weight());
                    }
                }
                println!("epoch: {} simulation step: {} -> herbis: {} carnis: {}",e+1,s+1,self.herbi.len(),self.carni.len());
            }//Sim Steps
            let herbi_keys: Vec<(i32, i32)> = self.herbi.keys().cloned().collect();
            let carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
            let mut next_gen_herbi: HashMap<(i32,i32), T> = HashMap::new();
            let mut next_gen_carni: HashMap<(i32,i32), E> = HashMap::new();
            //removing starved carnivores
            for c in carni_keys {
                if !self.carni.get_mut(&c).expect("no carni").has_enough_energy() {
                    self.carni.remove(&c);
                }
            }
            //removing starved herbivores
            for h in herbi_keys {
                if !self.herbi.get_mut(&h).expect("no herbi").has_enough_energy(){
                    self.herbi.remove(&h);
                }
            }
            if self.carni.len() <= 1 {
                println!("carnivores died out");
                break;
            }
            if self.herbi.len() <= 1 {
                println!("herbivores died out");
                break;
            }
            //removing plants
            self.plants.clear();

            //replacing plants
            for _ in 0..300 {
                let k = gen_pos();
                if !self.plants.contains_key(&k){
                    self.plants.insert(k, true);
                }
            };

            let herbi_keys: Vec<(i32, i32)> = self.herbi.keys().cloned().collect();
            let carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
            //placing herbivores
            for _ in 0..100{
                let k = gen_pos();
                let parent1 = herbi_keys.get(gen_vec_pos(herbi_keys.len())).expect("vec error");
                let parent2 = herbi_keys.get(gen_vec_pos(herbi_keys.len())).expect("vec error");
                if !next_gen_herbi.contains_key(&k){
                    next_gen_herbi.insert(k,
                         self.herbi.get(parent1)
                         .expect("no parent 1")
                         .crossover(self.herbi.get(parent2)// <--- CROSSOVER
                         .expect("no parent 2")));
                }
            };
            self.herbi = next_gen_herbi;
            //placing carnivoress
            for _ in 0..100{
                let k = gen_pos();
                let parent1 = carni_keys.get(gen_vec_pos(carni_keys.len())).expect("vec error");
                let parent2 = carni_keys.get(gen_vec_pos(carni_keys.len())).expect("vec error");
                if !next_gen_carni.contains_key(&k){
                    next_gen_carni.insert(k,
                         self.carni.get(parent1)
                         .expect("no parent 1")
                         .crossover(self.carni.get(parent2)// <--- CROSSOVER
                         .expect("no parent 2")));
                }
            };
            self.carni = next_gen_carni;
            //Epoch Output:

        }
        
    }
}

///gen_pos generates a random position
fn gen_pos() -> (i32, i32){
    
    let mut rng = rand::thread_rng();
    (rng.gen_range(-50..51), rng.gen_range(-50..51))
}

fn herbi_detect <T,E> (h: (i32,i32), carni: &HashMap<(i32,i32),E>, herbi: &HashMap<(i32,i32),T>, plants: &HashMap<(i32,i32),bool>) -> (i32,i32) 
    where E: Genome, T:Genome 
{
    let mut rng = rand::thread_rng();
    let k = (rng.gen_range(-1..2),rng.gen_range(-1..2));
    if carni.contains_key(&add_2x_tupel(k, h)) || herbi.contains_key(&add_2x_tupel(k, h)){
        return (0,0);
    }
    k
}

fn carni_detect <T,E> (h: (i32,i32), carni: &HashMap<(i32,i32),E>, herbi: &HashMap<(i32,i32),T>) -> (i32,i32) 
    where E: Genome, T:Genome 
{
    let mut rng = rand::thread_rng();
    (rng.gen_range(-1..2),rng.gen_range(-1..2))
}

fn add_2x_tupel(a:(i32,i32), b:(i32,i32)) -> (i32,i32){
    ( a.0 + b.0 , a.1 + b.1)
}

fn gen_vec_pos(max: usize)-> usize{
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}

fn main() {
    let mut sim:BasicSimulation<BasicGenome, BasicGenome> = BasicSimulation::new(10, 20, 5);
    sim.run();
}
