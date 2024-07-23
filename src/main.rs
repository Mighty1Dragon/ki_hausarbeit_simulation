use std::{collections::{btree_map::Keys, HashMap}, rc::Rc, thread::Thread};

use genome::{BasicGenome, Genome};
use rand::Rng;

mod genome;
mod ui;

trait Simulation{
    fn new(epochs: u16, sim_time: u16, mutation_chance: i32) -> Self;
    fn run(self);
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
        for i in 0..300 {
            let k = gen_pos();
            if !plants.contains_key(&k){
                plants.insert(k, true);
            }
        };
        //placing herbivores
        for i in 0..100{
            let k = gen_pos();
            if !herbi.contains_key(&k){
                herbi.insert(k, T::new(genome::EatingType::Herbivore));
            }
        };
        //placing carnivoress
        for i in 0..100{
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

    fn run(self) {
        
    }
}

///gen_pos generates a random position
fn gen_pos() -> (i32, i32){
    
    let mut rng = rand::thread_rng();
    (rng.gen_range(-50..51), rng.gen_range(-50..51))
}

fn main() {
    

}
