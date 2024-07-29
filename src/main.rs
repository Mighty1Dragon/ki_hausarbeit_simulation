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

        print_Field(&plants,&herbi,&carni);
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
                        carni_eat(&h, &mut self.carni, &mut self.herbi); // <--------SELECTION
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
                        carni_eat(&c, &mut self.carni, &mut self.herbi); // <--------SELECTION
                    }
                }
                println!("epoch: {} simulation step: {} -> herbis: {} carnis: {}",e+1,s+1,self.herbi.len(),self.carni.len());
                //print_Field(&self.plants, &self.herbi, &self.carni);
            }//Sim Steps
            let herbi_keys: Vec<(i32, i32)> = self.herbi.keys().cloned().collect();
            let carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();

            //removing starved carnivores <------SELECTION
            for c in carni_keys {
                if !self.carni.get_mut(&c).expect("no carni").has_enough_energy() {
                    self.carni.remove(&c);
                }
            }
            //removing starved herbivores <------SELECTION
            for h in herbi_keys {
                if !self.herbi.get_mut(&h).expect("no herbi").has_enough_energy(){
                    self.herbi.remove(&h);
                }
            }

            println!("remaining Herbivores: {}", self.herbi.len());
            println!("remaining Carnivores: {}", self.carni.len());
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

            //placing herbivores <----- CROSSOVER AND MUTATION
            self.herbi = place_genom(herbi_keys, &mut self.herbi, self.mutation_chance);

            //placing carnivoress <----- CROSSOVER AND MUTATION
            self.carni = place_genom(carni_keys, & mut self.carni, self.mutation_chance);
           
        }
        
    }
}

///gen_pos generates a random position
fn gen_pos() -> (i32, i32){
    
    let mut rng = rand::thread_rng();
    (rng.gen_range(-50..51), rng.gen_range(-50..51))
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn dir(&self) -> (i32,i32){
        match self {
            Direction::Left => (-1,0),
            Direction::Right => (1,0),
            Direction::Up => (0,-1),
            Direction::Down => (0,1),
        }
    }
    fn ord(&self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        }
    }
    fn get(num: usize) -> Direction{
        match num {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            3 => Direction::Down,
            _ => panic!("num to high")
        }
    }
}

fn calculate_reward(dir:&(i32,i32) ,a: &(i32,i32), b:&(i32,i32), reward: i32) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let c = (a.0 + dir.0, a.1 + dir.1);
    if c.0 > b.0 {
        x = c.0 - b.0;
    }else{
        x = b.0 - c.0;
    };
    if c.1 > b.1 {
        y = c.1 - b.1;
    }else{
        y = b.1 - c.1;
    };
    if x+y == 0 {
        return reward;
    }
    reward/(x+y)
}
fn calculate_distance_reward<T>(current: &T, g: &(i32,i32) , xy: &(i32,i32),directions: &mut [i32;4], num:u8)where T: Genome{
    //Left
    directions[Direction::Left.ord()] += calculate_reward(
        &Direction::Left.dir(), g, xy, current.get_eval(num));
    //Right
    directions[Direction::Right.ord()] += calculate_reward(
        &Direction::Right.dir(), g, xy, current.get_eval(num));
    //Up
    directions[Direction::Up.ord()] += calculate_reward(
        &Direction::Up.dir(), g, xy, current.get_eval(num));
    //Down
    directions[Direction::Down.ord()] += calculate_reward(
        &Direction::Down.dir(), g, xy, current.get_eval(num));
}

fn herbi_detect <T,E> (h: (i32,i32), carni: &HashMap<(i32,i32),E>, herbi: &HashMap<(i32,i32),T>, plants: &HashMap<(i32,i32),bool>) -> (i32,i32) 
    where E: Genome, T:Genome 
{
    let current_herbi = herbi.get(&h).expect("current herbi not available");
    let dr = current_herbi.get_detection_range().round() as i32;
    let mut directions = [0,0,0,0];
    for x in (&h.0 - dr)..(&h.0 + dr){
        for y in (&h.1 - dr)..(&h.1 + dr){
            if x.wrapping_add(y) > dr{//to make a more round detection window
                continue;
            }
            if carni.contains_key(&(x,y)){
                calculate_distance_reward(current_herbi, &h, &(x,y), &mut directions, 1);
            }
            if herbi.contains_key(&(x,y)) && !(x == h.0 && y == h.1){

                calculate_distance_reward(current_herbi, &h, &(x,y), &mut directions, 2);
            }
            if plants.contains_key(&(x,y)){
                calculate_distance_reward(current_herbi, &h, &(x,y), &mut directions, 3);
            }
        };
    };
    let mut rng = thread_rng();
    let r = rng.gen_range(0..4);
    //prevent loss of gene by collision
    for i in 0..directions.len(){
        if herbi.contains_key(&(h.0+Direction::get(i).dir().0, h.1+Direction::get(i).dir().1)){
            directions[i] = i32::min_value();
        }
    };    
    let mut choice = (directions[r], Direction::get(r));
    for i in 0..directions.len(){
        if directions[i] > choice.0{
            choice = (directions[i], Direction::get(i));
        }
    };  
    
    if choice.0 < 0 {
        return (0, 0);
    }
    (choice.1.dir().0, choice.1.dir().1)

}

fn carni_detect <T,E> (h: (i32,i32), carni: &HashMap<(i32,i32),E>, herbi: &HashMap<(i32,i32),T>) -> (i32,i32) 
    where E: Genome, T:Genome 
{
    let current_carni = carni.get(&h).expect("current carni not available");
    let dr = current_carni.get_detection_range().round() as i32;
    let mut directions = [0,0,0,0];
    for x in (&h.0 - dr)..(&h.0 + dr){
        for y in (&h.1 - dr)..(&h.1 + dr){
            if x.wrapping_add(y) > dr{//to make a more round detection window
                continue;
            }
            if carni.contains_key(&(x,y)) && !(x == h.0 && y == h.1){
                calculate_distance_reward(current_carni, &h, &(x,y), &mut directions, 1);
            }
            if herbi.contains_key(&(x,y)){

                calculate_distance_reward(current_carni, &h, &(x,y), &mut directions, 2);
            }
        };
         
    };
    let mut rng = thread_rng();
    let r = rng.gen_range(0..4);
    //prevent loss of gene by collision
    for i in 0..directions.len(){
        if carni.contains_key(&(h.0+Direction::get(i).dir().0, h.1+Direction::get(i).dir().1)){
            directions[i] = i32::min_value();
        }
    };
    let mut choice = (directions[r], Direction::get(r));
    for i in 0..directions.len(){
        if directions[i] > choice.0{
            choice = (directions[i], Direction::get(i));
        }
    };  
    if choice.0 < 0 {
        return (h.0, h.1);
    }
    (choice.1.dir().0, choice.1.dir().1)
}

fn add_2x_tupel(a:(i32,i32), b:(i32,i32)) -> (i32,i32){
    ( a.0.wrapping_add(b.0) , a.1.wrapping_add(b.1))
}

fn gen_vec_pos(max: usize)-> usize{
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}

fn print_Field<T,E>(plants: &HashMap<(i32,i32),bool>, herbi: &HashMap<(i32,i32),T>, carni: &HashMap<(i32,i32),E>){
    for y in -50..50 {
        for x in -50..50{
            if carni.contains_key(&(x,y)) {
                print!("C");
            }else if herbi.contains_key(&(x,y)) {
                print!("H")
            }else if plants.contains_key(&(x,y)) {
                print!("*");
            }else{
                print!("_")
            }
        }
        println!();
    }
}

fn carni_eat<T,E>(pos: &(i32,i32), carni: &mut HashMap<(i32,i32), E>, herbi: &mut HashMap<(i32,i32), T>)where T: Genome, E: Genome{
    let dead = herbi.remove(pos);
    carni.get_mut(pos)
    .expect("carni not existend")
    .increase_energy(
        dead.expect("herbi not existend").get_weight()
    );
}

fn place_genom<T>(keys: Vec<(i32,i32)>, map: &mut HashMap<(i32,i32), T>, chance: i32) -> HashMap<(i32,i32), T> where T: Genome{
    let mut next_gen: HashMap<(i32,i32), T> = HashMap::new();
    for _ in 0..100{
        let k = gen_pos();
        let parent1 = keys.get(gen_vec_pos(keys.len())).expect("vec error");
        let parent2 = keys.get(gen_vec_pos(keys.len())).expect("vec error");
        if !next_gen.contains_key(&k){
            next_gen.insert(k,
                 map.get(parent1)
                 .expect("no parent 1")
                 .crossover(map.get(parent2)// <--- CROSSOVER
                 .expect("no parent 2")));
            next_gen.get_mut(&k).expect("fail to mutate herbi").mutate(chance);//<-----MUTATE
        }
    };
    next_gen
}

fn main() {
    let mut sim:BasicSimulation<BasicGenome, BasicGenome> = BasicSimulation::new(40, 30, 5);
    sim.run();
}
