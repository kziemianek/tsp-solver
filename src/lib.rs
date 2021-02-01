use std::thread;
use time::Duration;

mod cli;
mod reader;
mod solver;

pub fn start() {
    let matches = cli::start().get_matches();
    let file = matches.value_of("file").unwrap().to_owned();
    let computation_duration: i64 = matches
        .value_of("duration")
        .unwrap()
        .to_owned()
        .parse()
        .unwrap();
    let alg = matches.value_of("alg").unwrap().to_owned();
    let runs: i32 = matches
        .value_of("runs")
        .unwrap()
        .to_owned()
        .parse()
        .unwrap();
    let parallel: bool = matches
        .value_of("parallel")
        .unwrap()
        .to_owned()
        .parse()
        .unwrap();

    let solutions = solve(&file, computation_duration, &alg, runs, parallel);
    solutions
        .iter()
        .for_each(|solution| println!("Solution score {}", solution.travel_distance));
}

fn solve(
    path: &str,
    duration: i64,
    alg: &str,
    runs: i32,
    parallel: bool,
) -> Vec<solver::TspSolution> {
    let mut solutions: Vec<solver::TspSolution> = vec![];
    if parallel {
        let cpus: i32 = num_cpus::get() as i32;
        generate_iterations_queue(runs, cpus)
            .iter()
            .for_each(|iteration_runs| {
                let mut joins: Vec<thread::JoinHandle<_>> = vec![];
                for _ in 0..*iteration_runs {
                    let alg = alg.to_owned();
                    let path = path.to_owned();
                    joins.push(thread::spawn(move || match alg.to_owned().as_str() {
                        "hill-climbing" => hill_climbing(&path, duration),
                        "simulated-annealing" => simulated_annealing(&path, duration),
                        "random-search" => random_search(&path, duration),
                        _ => panic!("Unknown alg!"),
                    }));
                }
                for join in joins {
                    solutions.push(join.join().unwrap());
                }
            });
    } else {
        for _x in 0..runs {
            match alg {
                "hill-climbing" => solutions.push(hill_climbing(path, duration)),
                "simulated-annealing" => solutions.push(simulated_annealing(path, duration)),
                "random-search" => solutions.push(random_search(path, duration)),
                _ => panic!("Unknown alg!"),
            }
        }
    }
    solutions
}

fn hill_climbing(path: &str, computation_time: i64) -> solver::TspSolution {
    let data = reader::read(path).unwrap();
    let distance_matrix = data.generate_distance_matrix();
    let mut instance = solver::TspInstance::new(distance_matrix);
    metaheuristics::hill_climbing::solve(&mut instance, Duration::seconds(computation_time))
}

fn simulated_annealing(path: &str, computation_time: i64) -> solver::TspSolution {
    let data = reader::read(path).unwrap();
    let distance_matrix = data.generate_distance_matrix();
    let mut instance = solver::TspInstance::new(distance_matrix);
    metaheuristics::simulated_annealing::solve(&mut instance, Duration::seconds(computation_time))
}

fn random_search(path: &str, computation_time: i64) -> solver::TspSolution {
    let data = reader::read(path).unwrap();
    let distance_matrix = data.generate_distance_matrix();
    let mut instance = solver::TspInstance::new(distance_matrix);
    metaheuristics::random_search::solve(&mut instance, Duration::seconds(computation_time))
}

fn generate_iterations_queue(runs: i32, cpus: i32) -> Vec<i32> {
    let full_iterations: i32 = ((runs / cpus) as f64).floor() as i32;
    let remaining_iterations: i32 = runs - full_iterations * cpus;
    let mut iterations: Vec<i32> = (0..full_iterations).map(|_x| cpus).collect();
    iterations.push(remaining_iterations);
    iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_iterations_queue() {
        let runs: i32 = 10;
        let cpus: i32 = 4;
        let expected: Vec<i32> = vec![4, 4, 2];

        assert_eq!(expected, generate_iterations_queue(runs, cpus));
    }
}
