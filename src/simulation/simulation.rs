/// in this module everything about the simulation and the world is defined
/// 
/// 
/// 
/// 

pub trait Simulation {
    fn new() -> Self;
    fn run(&mut self);
    fn print(&self);
    
}