use std::thread;
use time::Duration;

mod reader;
mod solver;

pub fn solve(
    path: &str,
    duration: i64,
    alg: &str,
    runs: i32,
    parallel: bool,
) -> Vec<Result<solver::TspSolution, String>> {
    let mut solutions: Vec<Result<solver::TspSolution, String>> = vec![];
    if parallel {
        let cpus: i32 = num_cpus::get() as i32;
        generate_iterations_queue(runs, cpus)
            .iter()
            .for_each(|iteration_runs| {
                let mut joins: Vec<thread::JoinHandle<_>> = vec![];
                for _ in 0..*iteration_runs {
                    let alg = alg.to_owned();
                    let path = path.to_owned();
                    joins.push(thread::spawn(move || perform(&alg, &path, duration)));
                }
                for join in joins {
                    match join.join() {
                        Ok(v) => solutions.push(v),
                        _ => solutions.push(Result::Err("Computation error...".to_owned())),
                    }
                }
            });
    } else {
        for _x in 0..runs {
            solutions.push(perform(alg, path, duration));
        }
    }
    solutions
}

fn perform(alg: &str, path: &str, computation_time: i64) -> Result<solver::TspSolution, String> {
    match reader::read(path) {
        Ok(data) => {
            let distance_matrix = data.generate_distance_matrix();
            let mut instance = solver::TspInstance::new(distance_matrix);
            match alg {
                "hill-climbing" => Result::Ok(metaheuristics::hill_climbing::solve(
                    &mut instance,
                    Duration::seconds(computation_time),
                )),
                "simulated-annealing" => Result::Ok(metaheuristics::simulated_annealing::solve(
                    &mut instance,
                    Duration::seconds(computation_time),
                )),
                "random-search" => Result::Ok(metaheuristics::random_search::solve(
                    &mut instance,
                    Duration::seconds(computation_time),
                )),
                _ => Result::Err("Unknown alg!".to_owned()),
            }
        }
        Err(v) => Result::Err(v.to_string()),
    }
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
