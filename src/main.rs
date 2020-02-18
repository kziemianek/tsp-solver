use std::thread;
use tspsolver::solver;

fn main() {
    let t1 = thread::spawn(move || {
        let solution: solver::TspSolution = tspsolver::hill_climbing(&"instances/ym7663.tsp");
        println!("Solution hill climbing: {}", solution.travel_distance);
    });

    let t2 = thread::spawn(move || {
        let solution: solver::TspSolution = tspsolver::simulated_annealing(&"instances/ym7663.tsp");
        println!("Solution simulated annealing: {}", solution.travel_distance);
    });

    let t3 = thread::spawn(move || {
        let solution: solver::TspSolution = tspsolver::random_search(&"instances/ym7663.tsp");
        println!("Solution random search: {}", solution.travel_distance);
    });

    let _ = t1.join();
    let _ = t2.join();
    let _ = t3.join();
}
