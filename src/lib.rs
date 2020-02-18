use time::Duration;

mod reader;
pub mod solver;

pub fn hill_climbing(path: &str) -> solver::TspSolution {
    let data = reader::read(path).unwrap();
    let distance_matrix = data.generate_distance_matrix();
    let mut instance = solver::TspInstance::new(distance_matrix);
    let solution = metaheuristics::hill_climbing::solve(&mut instance, Duration::minutes(2));
    solution
}

pub fn simulated_annealing(path: &str) -> solver::TspSolution {
    let data = reader::read(path).unwrap();
    let distance_matrix = data.generate_distance_matrix();
    let mut instance = solver::TspInstance::new(distance_matrix);
    let solution = metaheuristics::simulated_annealing::solve(&mut instance, Duration::minutes(2));
    solution
}

pub fn random_search(path: &str) -> solver::TspSolution {
    let data = reader::read(path).unwrap();
    let distance_matrix = data.generate_distance_matrix();
    let mut instance = solver::TspInstance::new(distance_matrix);
    let solution = metaheuristics::random_search::solve(&mut instance, Duration::minutes(2));
    solution
}
