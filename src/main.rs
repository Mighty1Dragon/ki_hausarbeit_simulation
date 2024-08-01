use core::time;
use std::{borrow::Borrow, collections:: HashMap, fs:: File, io::{stdout, Write}, thread};

use genome::{BasicGenome, Genome};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

mod genome;

const PLANT_ENERGY: f32 = 1.0;
const WATCHING: bool = false;
const MILLIS_PER_FRAME: u64 = 1000; //in milliseconds
const MEAT_EFFICIENCY: f32 = 2.0;
const STRENGTH_CONTEST: bool = true;
const CARNI_EXTRA_MUTATION_CHANCE: i32 = 0;
const HERBI_EXTRA_MUTATION_CHANCE: i32 = 0;
const SLOW_PLANT_DECREASE: i32 = 3;
const HALF_PLANT_AT: u16 = 100;

const HERBI_NUM: i32 = 100;
const CARNI_NUM: i32 = 100;

trait Simulation{
    fn new(epochs: u16, sim_time: u16, mutation_chance: i32, file: File) -> Self;
    fn run(&mut self);
}
///for evaluation purposes
struct SimulationResult{
    epoch: u16,
    average_herbi: i32,
    average_carni: i32,
    die_out: Option<genome::EatingType>,
    average_herbi_start_attributes: (f32,f32,f32,f32,i32,i32,i32),//w s p d 1 2 3
    average_carni_start_attributes: (f32,f32,f32,f32,i32,i32,i32),//w s p d 1 2 3
    average_herbi_end_attributes: (f32,f32,f32,f32,i32,i32,i32),//w s p d 1 2 3
    average_carni_end_attributes: (f32,f32,f32,f32,i32,i32,i32),//w s p d 1 2 3
}
impl SimulationResult {
    fn new() -> SimulationResult{
        SimulationResult{
            epoch: 0,
            average_carni: 0,
            average_herbi: 0,
            die_out: None,
            average_herbi_start_attributes: (0.0,0.0,0.0,0.0,0,0,0),
            average_carni_start_attributes: (0.0,0.0,0.0,0.0,0,0,0),
            average_herbi_end_attributes: (0.0,0.0,0.0,0.0,0,0,0),
            average_carni_end_attributes: (0.0,0.0,0.0,0.0,0,0,0),
        }
    }

    fn get_average_herbi(&self) -> i32 {
        self.average_herbi / self.epoch as i32
    }

    fn get_average_carni(&self) -> i32 {
        self.average_carni / self.epoch as i32
    }

    fn get_ahsa(&self) -> (f32,f32,f32,f32,i32,i32,i32){
        let a = self.borrow().average_herbi_start_attributes;
        self.average_7tupel(a)
    }
    fn get_acsa(&self) -> (f32,f32,f32,f32,i32,i32,i32){
        let a = self.borrow().average_carni_start_attributes;
        self.average_7tupel(a)
    }
    fn get_ahea(&self) -> (f32,f32,f32,f32,i32,i32,i32){
        let a = self.borrow().average_herbi_end_attributes;
        self.average_7tupel(a)
    }
    fn get_acea(&self) -> (f32,f32,f32,f32,i32,i32,i32){
        let a = self.borrow().average_carni_end_attributes;
        self.average_7tupel(a)
    }

    fn average_7tupel(&self, a: (f32,f32,f32,f32,i32,i32,i32)) -> (f32,f32,f32,f32,i32,i32,i32){
        //let a = self.borrow().average_herbi_start_attributes;
        //(a.0/self.epoch as f32,a.1/self.epoch as f32,a.2/self.epoch as f32,a.3/self.epoch as f32,a.4/self.epoch as i32,a.5/self.epoch as i32,a.6/self.epoch as i32,)
        a
    }
}

struct BasicSimulation <T : Genome, E : Genome>{
    epochs: u16,
    sim_time: u16,
    mutation_chance: i32,
    file: File,
    plants: HashMap<(i32,i32), bool>,
    herbi: HashMap<(i32, i32), T>,
    carni: HashMap<(i32, i32), E>,
    res: SimulationResult,
}

impl<T : genome::Genome,E : genome::Genome> Simulation for BasicSimulation<T, E>{
    fn new(epochs: u16, sim_time: u16, mutation_chance: i32, mut file: File) -> Self {
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
        for _ in 0..HERBI_NUM{
            let k = gen_pos();
            if !herbi.contains_key(&k){
                herbi.insert(k, T::new(genome::EatingType::Herbivore));
            }
        };
        //placing carnivoress
        for _ in 0..CARNI_NUM{
            let k = gen_pos();
            if !carni.contains_key(&k){
                carni.insert(k, E::new(genome::EatingType::Carnivore));
            }
        };

        file_print(&mut file,format!("Simulation Start:\n"));
        //print_Field(&plants,&herbi,&carni,&mut file);
        BasicSimulation {
            epochs, sim_time, mutation_chance, file,
            plants, herbi, carni, res: SimulationResult::new()
        }
    }

    fn run(&mut self){
        file_print(&mut self.file, format!("EPOCHS: {}\nSIM_TIME: {}\nMUTATION_CHANCE: {}\nCARNI_EXTRA: {}\nHERBI_EXTRA: {}\n",
            self.epochs, self.sim_time, self.mutation_chance, CARNI_EXTRA_MUTATION_CHANCE, HERBI_EXTRA_MUTATION_CHANCE
        ));
        file_print(&mut self.file, format!("PLANT_ENERGY: {}\nMEAT_EFFICIENCY: {}\nSTRENGTH_CONTEST: {}\nPLANT DECREASE: {}\nPLANTS_GET_HALFED_AT: {}\n", 
            PLANT_ENERGY, MEAT_EFFICIENCY, STRENGTH_CONTEST, SLOW_PLANT_DECREASE, HALF_PLANT_AT
        ));
        for e in 0..self.epochs{
            self.res.epoch += 1;
            //Epoch Output
            file_print(&mut self.file, format!("###########################\n"));
            file_print(&mut self.file, format!("------EPOCH: {}---------\n", e+1));
            file_print(&mut self.file, format!("###########################\n"));
            let herbi_keys: Vec<(i32,i32)> = self.herbi.keys().cloned().collect();
            let carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
            self.res.average_herbi_end_attributes = (0.0,0.0,0.0,0.0,0,0,0);
            self.res.average_carni_end_attributes = (0.0,0.0,0.0,0.0,0,0,0);
            let herbi_len = herbi_keys.len();
            let carni_len = carni_keys.len();
    
            for g in herbi_keys{
                let h = self.herbi.get(&g).expect("herbi not available");
                file_print(&mut self.file,format!("{}\n",h.to_string()));
                if e == 1 {
                    
                    self.res.average_herbi_start_attributes = add_7_tupel(self.res.average_herbi_start_attributes, 
                        (h.get_weight(),h.get_speed(),h.get_power(),h.get_detection_range(),h.get_eval(1),h.get_eval(2),h.get_eval(3))
                    );

                }
                self.res.average_herbi_end_attributes = add_7_tupel(self.res.average_herbi_end_attributes, 
                    (h.get_weight(),h.get_speed(),h.get_power(),h.get_detection_range(),h.get_eval(1),h.get_eval(2),h.get_eval(3))
                );
            }
            self.res.average_herbi_start_attributes = average_7_tupel(self.res.average_herbi_start_attributes, herbi_len);
            self.res.average_herbi_end_attributes = average_7_tupel(self.res.average_herbi_end_attributes, herbi_len);
            for g in carni_keys{
                let h = self.carni.get(&g).expect("carni not available");
                file_print(&mut self.file,format!("{}\n",h.to_string()));
                
                if e == 1 {
                    let h = self.carni.get(&g).expect("carni not available");
                    self.res.average_carni_start_attributes = add_7_tupel(self.res.average_carni_start_attributes, 
                        (h.get_weight(),h.get_speed(),h.get_power(),h.get_detection_range(),h.get_eval(1),h.get_eval(2),h.get_eval(3))
                    );
                }
                self.res.average_carni_end_attributes = add_7_tupel(self.res.average_carni_end_attributes, 
                    (h.get_weight(),h.get_speed(),h.get_power(),h.get_detection_range(),h.get_eval(1),h.get_eval(2),h.get_eval(3))
                );
            }
            self.res.average_carni_start_attributes = average_7_tupel(self.res.average_carni_start_attributes, carni_len);
            self.res.average_carni_end_attributes = average_7_tupel(self.res.average_carni_end_attributes, carni_len);
            for s in 0..self.sim_time{
                if WATCHING {
                    animate(&self.plants, &self.herbi, &self.carni);
                }
                let mut rng = thread_rng();
                let mut rng2 = thread_rng();
                let mut herbi_keys: Vec<(i32,i32)> = self.herbi.keys().cloned().collect();
                let mut carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
                herbi_keys.shuffle( &mut rng);
                carni_keys.shuffle( &mut rng2);
                //Herbi move
                for oh in herbi_keys{
                    let mut h = oh;
                    //move as often as you have speed
                    let speed = self.herbi.get(&h).expect("no herbi: this is a bug i couldn't fix. just restart").get_speed().round() as i32;
                    for _ in 0..speed {
                        
                        if self.carni.contains_key(&h) && 0.0 < compare_strength(self.carni.get(&h).expect("com str"),self.herbi.get(&h).expect("com str")) {
                            carni_eat(&h, &mut self.carni, &mut self.herbi); // <--------SELECTION
                            break;
                        }
                        //choosing direction
                        let herbi_direction = herbi_detect(h, &self.carni, &self.herbi, &self.plants);
                        let temp = self.herbi.remove(&h).expect("herbi does not exist");
                        //moving a step
                        let new_pos = add_2x_tupel(herbi_direction, h);
                        h = new_pos;
                        self.herbi.insert(new_pos, temp);

                        if self.plants.contains_key(&new_pos){
                            self.plants.remove_entry(&new_pos);
                            self.herbi.get_mut(&new_pos).expect("herbi does not exist").increase_energy(PLANT_ENERGY);
                        }
                    }
                }
                let mut herbi_keys: Vec<(i32,i32)> = self.herbi.keys().cloned().collect();
                let mut carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();
                herbi_keys.shuffle( &mut rng);
                carni_keys.shuffle( &mut rng2);
                //Carni Move
                for oc in carni_keys {
                    let mut c = oc;
                    if !self.carni.contains_key(&c){
                        continue;
                    }
                    let speed = self.carni.get(&c).expect("no carni: this is a bug i couldn't fix. just restart").get_speed().round() as i32;
                    //move as often as you have speed
                    for _ in 0..speed {
                        let carni_direction = carni_detect(c, &self.carni, &self.herbi);
                        let temp = self.carni.remove(&c).expect("no carni :(");
                        let new_pos = add_2x_tupel(carni_direction, c);
                        c = new_pos;
                        self.carni.insert(new_pos, temp);
                        if self.herbi.contains_key(&new_pos) && 0.0 < compare_strength(self.carni.get(&c).expect("com str"),self.herbi.get(&c).expect("com str")){
                            carni_eat(&new_pos, &mut self.carni, &mut self.herbi); // <--------SELECTION
                        }
                    }
                }
                file_print(&mut self.file,format!("epoch: {} simulation step: {} -> herbis: {} carnis: {}\n",e+1,s+1,self.herbi.len(),self.carni.len()));
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
            self.res.average_carni += self.carni.len() as i32;
            self.res.average_herbi += self.herbi.len() as i32;
            
            let herbi_keys: Vec<(i32, i32)> = self.herbi.keys().cloned().collect();
            let carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();

            file_print(&mut self.file, format!("surviving genes:\n"));
            for g in herbi_keys{
                file_print(&mut self.file,format!("{}\n",self.herbi.get(&g).expect("herbi not available").to_string()));
            }
            for g in carni_keys{
                file_print(&mut self.file,format!("{}\n",self.carni.get(&g).expect("herbi not available").to_string()));
            }

            file_print(&mut self.file,format!("remaining Herbivores: {}\n", self.herbi.len()));
            file_print(&mut self.file,format!("remaining Carnivores: {}\n", self.carni.len()));
            if self.carni.len() <= 1 {
                file_print(&mut self.file,format!("carnivores died out\n"));
                self.res.die_out = Some(genome::EatingType::Carnivore);
                break;
            }
            if self.herbi.len() <= 1 {
                file_print(&mut self.file,format!("herbivores died out\n"));
                self.res.die_out = Some(genome::EatingType::Herbivore);
                break;
            }
            //removing plants
            self.plants.clear();
            let mut  plants_to_place = 300 - (SLOW_PLANT_DECREASE * e as i32);
            if e >= HALF_PLANT_AT {
                plants_to_place = plants_to_place/2
            }
            //replacing plants
            for _ in 0..plants_to_place {
                let k = gen_pos();
                if !self.plants.contains_key(&k){
                    self.plants.insert(k, true);
                }
            };

            let herbi_keys: Vec<(i32, i32)> = self.herbi.keys().cloned().collect();
            let carni_keys: Vec<(i32, i32)> = self.carni.keys().cloned().collect();

            //placing herbivores <----- CROSSOVER AND MUTATION
            self.herbi = place_genom(herbi_keys, &mut self.herbi, self.mutation_chance + HERBI_EXTRA_MUTATION_CHANCE, HERBI_NUM);

            //placing carnivoress <----- CROSSOVER AND MUTATION
            self.carni = place_genom(carni_keys, & mut self.carni, self.mutation_chance + CARNI_EXTRA_MUTATION_CHANCE, CARNI_NUM);
           
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
    let x;
    let y;
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

fn add_7_tupel( a: (f32,f32,f32,f32,i32,i32,i32), b: (f32,f32,f32,f32,i32,i32,i32)) -> (f32,f32,f32,f32,i32,i32,i32){
    (a.0 + b.0, b.1 + a.1 , a.2 + b.2, a.3 + b.3, a.4 + b.4 , a.5 + b.5, a.6 + a.6)
}

fn gen_vec_pos(max: usize)-> usize{
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}

fn animate<T,E>(plants: &HashMap<(i32,i32),bool>, herbi: &HashMap<(i32,i32),T>, carni: &HashMap<(i32,i32),E>){
    match stdout().flush(){
        Ok(_) => {
            for y in -50..50 {
                for x in -50..50{
                    if carni.contains_key(&(x,y)) {
                        print!("C");
                    }else if herbi.contains_key(&(x,y)) {
                        print!("H");
                    }else if plants.contains_key(&(x,y)) {
                        print!("*");
                    }else{
                        print!("_");
                    }
                }
                println!();
                
            }
        }
        Err(e) => panic!("in animate: {}",e)
    };
    thread::sleep(time::Duration::from_millis(MILLIS_PER_FRAME))
}

fn carni_eat<T,E>(pos: &(i32,i32), carni: &mut HashMap<(i32,i32), E>, herbi: &mut HashMap<(i32,i32), T>)where T: Genome, E: Genome{
    let dead = herbi.remove(pos);
    carni.get_mut(pos)
    .expect("carni not existend")
    .increase_energy(
        calculate_meat_efficiency(dead.expect("herbi not existend").get_weight())
    );
}

fn place_genom<T>(keys: Vec<(i32,i32)>, map: &mut HashMap<(i32,i32), T>, chance: i32, genom_num: i32) -> HashMap<(i32,i32), T> where T: Genome{
    let mut next_gen: HashMap<(i32,i32), T> = HashMap::new();
    for _ in 0..genom_num{
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
fn file_print(file: &mut File, string:String){
    file.write(string.as_bytes()).expect("write went wrong");
}

fn compare_strength<T: Genome,E: Genome>(carni: &E, herbi: &T) -> f32 {
    if STRENGTH_CONTEST {
        carni.get_power() - herbi.get_power()
    }else{
        1.0
    }
}

fn calculate_meat_efficiency(weight: f32) -> f32{
    weight * MEAT_EFFICIENCY
}

const SINGLE: bool = false;
fn main() {
    let folder = "slow_plant_decrease/";
    let file_name = "test";
    let mut completed = 0;
    let mut h_died_out = 0;
    let mut c_died_out = 0;
    let mut ha = 0;
    let mut ca = 0;
    let epochs = 40;
    let num_of_simulations = 100;
    let mut res_file = File::create(format!("data/{}.txt", format!("{}res",folder))).expect("file problem");
    if SINGLE {
        let file = File::create(format!("data/{}{}.txt",folder, file_name)).expect("file problem");
        let mut sim:BasicSimulation<BasicGenome, BasicGenome> = BasicSimulation::new(40, 30, 150, file);
        sim.run();
    }else{
        for s in 0..num_of_simulations{
            let file = File::create(format!("data/{}.txt", format!("{}test{}",folder,s))).expect("file problem");
            let mut sim:BasicSimulation<BasicGenome, BasicGenome> = BasicSimulation::new(epochs, 30, 150, file);
            sim.run();
            if sim.res.epoch == epochs {
                completed += 1;
            }
            let die_out_txt = match sim.res.die_out.clone() {
                Some(x) => match x {
                    genome::EatingType::Carnivore => {
                        c_died_out += 1;
                        "Carnivore died out".to_owned()
                    },
                    genome::EatingType::Herbivore =>  {
                        h_died_out += 1;
                        "Herbivore died out".to_owned()
                    },
                    genome::EatingType::Omnivore => panic!("Omnivore not supported"),
                }
                None => "working".to_owned(),
            };
            let temp_ha = sim.res.get_average_herbi();
            let temp_ca = sim.res.get_average_carni();
            ha += temp_ha;
            ca += temp_ca;
            println!("Simulation number: {} -> stoped at: {} - average herbi: {} average carni: {} -- {}", s, sim.res.epoch, temp_ha, temp_ca, die_out_txt);
            res_file.write(format!("Simulation number: {} -> stoped at: {} - average herbi: {} average carni: {} -- {}\n", s, sim.res.epoch,temp_ha,temp_ca, die_out_txt).as_bytes()).expect("res file fail!");
            
            let o = sim.res.get_ahsa();
            res_file.write(get_7tupel_format("Average Herbi Start",o).as_bytes()).expect("res file fail!");
            let o = sim.res.get_acsa();
            res_file.write(get_7tupel_format("Average Carni Start",o).as_bytes()).expect("res file fail!");
            let o = sim.res.get_ahea();
            res_file.write(get_7tupel_format("Average Herbi End",o).as_bytes()).expect("res file fail!");
            let o = sim.res.get_acea();
            res_file.write(get_7tupel_format("Average Carni End",o).as_bytes()).expect("res file fail!");
        }
        println!("Simulations completed: {} Herbivores died out: {} times and Carnivores died out: {} times", completed, h_died_out, c_died_out);
        res_file.write(format!("Simulations completed: {} Herbivores died out: {} times and Carnivores died out: {} times \nHerbi average: {} Canri average: {}\n", completed,h_died_out,c_died_out, ha/num_of_simulations, ca/num_of_simulations).as_bytes()).expect("res file fail!");
        
    }
}

fn get_7tupel_format(s: &str, o: (f32,f32,f32,f32,i32,i32,i32)) -> String {
    format!("{} w: {}, s: {}, p: {}, d: {}, eval- 1: {}, 2: {}, 3: {} \n", s,o.0,o.1,o.2,o.3,o.4,o.5,o.6)
}

fn average_7_tupel(a: (f32,f32,f32,f32,i32,i32,i32), b: usize) -> (f32,f32,f32,f32,i32,i32,i32){
    (a.0/b as f32,a.1/b as f32,a.2/b as f32,a.3/b as f32, a.4/b as i32, a.5/b as i32, a.6/b as i32)
}