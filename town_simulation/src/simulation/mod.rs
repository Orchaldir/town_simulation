use crate::SimulationData;

pub fn simulate_year(data: &mut SimulationData) {
    println!("Simulate year {}", data.date.get_year());

    data.date.increase_year();
}
