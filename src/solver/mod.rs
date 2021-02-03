extern crate rand;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use pathfinding;
use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, FromStr};

pub struct TspSolution {
    pub city_order: Vec<u32>,
    pub travel_distance: Decimal,
}

impl TspSolution {
    fn new(city_order: Vec<u32>, travel_distance: Decimal) -> TspSolution {
        TspSolution {
            city_order,
            travel_distance,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node {
    pub index: u32,
    pub x: Decimal,
    pub y: Decimal,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeTuple(Decimal, Decimal, Vec<Node>, u32, Vec<Vec<Decimal>>);


impl NodeTuple {
    fn neighbours(&self) -> Vec<(NodeTuple, Decimal)> {
        self.2.iter()
            .filter(|node| !node.x.eq(&self.0) && !node.y.eq(&self.1))
            .map(|node| (NodeTuple(node.x, node.y, self.2.to_vec(), node.index, self.4.to_vec()), self.4.get((node.index-1) as usize).unwrap().get((self.3-1) as usize).unwrap().clone()))
            .collect()
    }
}


impl Node {
    pub fn new(index: u32, x: Decimal, y: Decimal) -> Node {
        Node { index, x, y }
    }
    pub fn distance_to(&self, node: &Node) -> Decimal {
        (((node.x - &self.x).powi(2) + (node.y - &self.y).powi(2)).sqrt()).unwrap()
    }
}


#[allow(dead_code)]
enum InitialSolutionGenerationAlgorithm {
    SHUFFLE,
    DIJKSTRA,
}

pub struct TspInstance {
    pub distance_matrix: Vec<Vec<Decimal>>,
    pub rng: ThreadRng,
    pub nodes: Vec<Node>
}

impl TspInstance {
    pub fn new(nodes: Vec<Node>) -> TspInstance {
        let rng = thread_rng();
        let distance_matrix = TspInstance::generate_distance_matrix(&nodes);
        TspInstance {
            distance_matrix,
            rng,
            nodes
        }
    }

    fn generate_distance_matrix(nodes: &Vec<Node>) -> Vec<Vec<Decimal>> {
        let nodes_number = nodes.len();
        let mut distance_matrix: Vec<Vec<Decimal>> = Vec::with_capacity(nodes_number);
        for node in nodes {
            let mut distances: Vec<Decimal> = Vec::with_capacity(nodes_number);
            for coord2 in nodes {
                distances.push(node.distance_to(&coord2));
            }
            distance_matrix.push(distances);
        }
        distance_matrix
    }
}

impl metaheuristics::Metaheuristics<TspSolution> for TspInstance {
    fn clone_candidate(&mut self, t: &TspSolution) -> TspSolution {
        let city_order = t.city_order.to_vec();
        let travel_distance = t.travel_distance;
        TspSolution::new(city_order, travel_distance)
    }
    fn generate_candidate(&mut self) -> TspSolution {
        let city_order = generate_initial_solution_city_order(self, InitialSolutionGenerationAlgorithm::DIJKSTRA);
        let travel_distance = calculate_travel_distance(&city_order, &self.distance_matrix);
        TspSolution::new(city_order, travel_distance)
    }
    fn rank_candidate(&mut self, solution: &TspSolution) -> f64 {
        // !println!("{}", solution.travel_distance.to_string());
        // (Decimal::max_value() - solution.travel_distance).to_f64().unwrap()
        solution.travel_distance.to_f64().unwrap()
    }
    fn tweak_candidate(&mut self, solution: &TspSolution) -> TspSolution {
        let first_city_index: usize = self.rng.gen_range(0, solution.city_order.len());
        let second_city_index: usize = self.rng.gen_range(0, solution.city_order.len());
        let city_order = swap_cities(first_city_index, second_city_index, &solution.city_order);
        let travel_distance = calculate_travel_distance(&city_order, &self.distance_matrix);
        TspSolution::new(city_order, travel_distance)
    }
}

fn generate_initial_solution_city_order(instance: &mut TspInstance, algorithm: InitialSolutionGenerationAlgorithm) -> Vec<u32> {
    match algorithm {
        InitialSolutionGenerationAlgorithm::SHUFFLE => {
            let mut city_order: Vec<u32> = (0..instance.distance_matrix.len())
                .map(|n| n as u32)
                .collect();
            city_order.shuffle(&mut instance.rng);
            city_order
        },
        InitialSolutionGenerationAlgorithm::DIJKSTRA => {
            let nodes: Vec<NodeTuple> = instance.nodes.iter().map(|node| {
                NodeTuple(node.x, node.y, instance.nodes.iter().filter(|&_node| !node.eq(_node)).map(|node| node.clone()).collect(), node.index, instance.distance_matrix.to_vec())
            }).collect();
            pathfinding::dijkstra(&nodes[0], |instance| instance.neighbours(),  |n| n == &nodes[0])
                .unwrap().0.iter().map(|node| node.3-1).collect()
        }
    }
}

fn calculate_travel_distance(city_order: &[u32], distance_matrix: &[Vec<Decimal>]) -> Decimal {
    let mut score: Decimal = Decimal::from_str("0.0").unwrap();
    let first_city = *city_order.get(0).unwrap();
    let mut prev_city: u32 = first_city;
    // sum up distances from cities
    for city in city_order {
        let dist: &Decimal = distance_matrix
            .get(prev_city as usize)
            .unwrap()
            .get(*city as usize)
            .unwrap();
        score += dist;
        prev_city = city.to_owned();
    }
    // add distance from from last city to first
    score += distance_matrix
        .get(prev_city as usize)
        .unwrap()
        .get(first_city as usize)
        .unwrap();
    score
}

fn swap_cities(first_city: usize, second_city: usize, cities_order: &[u32]) -> Vec<u32> {
    let mut new_order = cities_order.to_owned();
    get_pairs_to_swap(first_city, second_city)
        .iter()
        .for_each(|pair| new_order.swap(pair.0, pair.1));
    new_order
}

fn get_pairs_to_swap(first: usize, second: usize) -> Vec<(usize, usize)> {
    // 0 smaller 1 larger
    let ordered = match first > second {
        true => (second, first),
        false => (first, second),
    };
    // [0 1 2 3 4 5 6 7 8 9]
    // first: 7, second: 2
    // smaller: 2, larger: 7
    // . . 2 . . . . 7 . . [(2,7)]
    // . . 2 3 . . 6 7 . . [(2,7), (3,6)]
    // . . 2 3 4 5 6 7 . . [(2,7), (3,6), (4,5)]
    let pairs_to_swap_count: usize = (((ordered.1 - ordered.0) / 2) as f64).floor() as usize;
    let pairs_to_swap: Vec<(usize, usize)> = (0..=pairs_to_swap_count)
        .map(|x| ((ordered.0 + x), (ordered.1 - x)))
        .collect();
    pairs_to_swap
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use crate::solver::{TspInstance, Node};
    use rust_decimal::prelude::FromStr;

    #[test]
    fn calculate_travel_distance_test() {
        let city_order = vec![1, 0, 2];
        let distance_matrix: Vec<Vec<Decimal>> = vec![
            vec![Decimal::from_str("0.0").unwrap(), Decimal::from_str("3.9").unwrap(), Decimal::from_str("6.44").unwrap()],
            vec![Decimal::from_str("3.9").unwrap(), Decimal::from_str("0.0").unwrap(), Decimal::from_str("4.7").unwrap()],
            vec![Decimal::from_str("6.44").unwrap(), Decimal::from_str("4.7").unwrap(), Decimal::from_str("0.0").unwrap()],
        ];

        let rank = super::calculate_travel_distance(&city_order, &distance_matrix);
        assert_eq!(rank, Decimal::from_str("15.04").unwrap());
    }

    #[test]
    fn swap_cities_test() {
        let city_order = vec![1, 0, 2, 4, 3];
        let new_order = super::swap_cities(0, 3, &city_order);

        assert_eq!(new_order, vec![4, 2, 0, 1, 3])
    }

    #[test]
    fn get_pairs_to_swap_test_1() {
        let first_idx: usize = 2;
        let second_idx: usize = 10;
        let expected_pairs = vec![
            (2, 10),
            (3, 9),
            (4, 8),
            (5, 7),
            // todo: we can omit 6,6 pair as swapping them doesn't change result
            (6, 6),
        ];
        assert_eq!(
            expected_pairs,
            super::get_pairs_to_swap(first_idx, second_idx)
        );
    }

    #[test]
    fn get_pairs_to_swap_test_2() {
        let first_idx: usize = 2;
        let second_idx: usize = 9;
        let expected_pairs = vec![(2, 9), (3, 8), (4, 7), (5, 6)];
        assert_eq!(
            expected_pairs,
            super::get_pairs_to_swap(first_idx, second_idx)
        );
    }

    #[test]
    fn get_pairs_to_swap_test_3() {
        let first_idx: usize = 10;
        let second_idx: usize = 2;
        let expected_pairs = vec![
            (2, 10),
            (3, 9),
            (4, 8),
            (5, 7),
            // todo: we can omit 6,6 pair as swapping them doesn't change result
            (6, 6),
        ];
        assert_eq!(
            expected_pairs,
            super::get_pairs_to_swap(first_idx, second_idx)
        );
    }

    #[test]
    fn get_pairs_to_swap_test_4() {
        let first_idx: usize = 9;
        let second_idx: usize = 2;
        let expected_pairs = vec![(2, 9), (3, 8), (4, 7), (5, 6)];
        assert_eq!(
            expected_pairs,
            super::get_pairs_to_swap(first_idx, second_idx)
        );
    }
    //
    // #[test]
    // fn generate_initial_solution_city_order_test() {
    //     let mut instance = TspInstance::new(vec![
    //         Node::new(1,Decimal::from_str(11003.611100), Decimal::from_str(42102.500000)),
    //         Node::new(2,Decimal::from_str(11108.611100), Decimal::from_str(42373.888900)),
    //         Node::new(3, Decimal::from_str(11133.333300), Decimal::from_str(42885.833300))
    //     ]);
    //
    //     assert_ne!(super::generate_initial_solution_city_order(&mut instance, super::InitialSolutionGenerationAlgorithm::DIJKSTRA), {})
    //
    //
    //
    // }


}
