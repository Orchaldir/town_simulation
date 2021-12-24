use crate::generation::number::RandomNumberGenerator;
use crate::simulation::character::death::simulate_death;
use crate::simulation::character::marriage::simulate_marriage;
use crate::SimulationData;

pub mod character;

pub fn simulate_year(data: &mut SimulationData) {
    println!("Simulate year {}", data.date.get_year());

    let rng = RandomNumberGenerator::Hash;

    simulate_marriage(data, &rng);
    simulate_death(data, &rng);

    data.date.increase_year();
}
