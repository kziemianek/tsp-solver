extern crate rand;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct TspSolution {
    pub city_order: Vec<u32>,
    pub travel_distance: f32,
}

impl TspSolution {
    fn new(city_order: Vec<u32>, travel_distance: f32) -> TspSolution {
        TspSolution {
            city_order,
            travel_distance,
        }
    }
}

pub struct TspInstance {
    pub distance_matrix: Vec<Vec<f32>>,
    pub rng: ThreadRng,
}

impl TspInstance {
    pub fn new(distance_matrix: Vec<Vec<f32>>) -> TspInstance {
        let rng = thread_rng();
        TspInstance {
            distance_matrix,
            rng,
        }
    }
}

impl metaheuristics::Metaheuristics<TspSolution> for TspInstance {
    fn clone_candidate(&mut self, t: &TspSolution) -> TspSolution {
        let city_order = t.city_order.to_vec();
        let travel_distance = t.travel_distance;
        TspSolution::new(city_order, travel_distance)
    }
    fn generate_candidate(&mut self) -> TspSolution {
        let mut city_order: Vec<u32> = (0..self.distance_matrix.len()).map(|n| n as u32).collect();
        city_order.shuffle(&mut self.rng);
        let travel_distance = calculate_travel_distance(&city_order, &self.distance_matrix);
        TspSolution::new(city_order, travel_distance)
    }
    fn rank_candidate(&mut self, solution: &TspSolution) -> f64 {
        std::f64::MAX - solution.travel_distance as f64
    }
    fn tweak_candidate(&mut self, solution: &TspSolution) -> TspSolution {
        let first_city_index: usize = self.rng.gen_range(0, solution.city_order.len());
        let second_city_index: usize = self.rng.gen_range(0, solution.city_order.len());
        let city_order = swap_cities(first_city_index, second_city_index, &solution.city_order);
        let travel_distance = calculate_travel_distance(&city_order, &self.distance_matrix);
        TspSolution::new(city_order, travel_distance)
    }
}

fn calculate_travel_distance(city_order: &[u32], distance_matrix: &[Vec<f32>]) -> f32 {
    let mut score: f32 = 0.0;
    let first_city = *city_order.get(0).unwrap();
    let mut prev_city: u32 = first_city;
    // sum up distances from cities
    for city in city_order {
        let dist: &f32 = distance_matrix
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

    #[test]
    fn calculate_travel_distance_test() {
        let city_order = vec![1, 0, 2];
        let distance_matrix: Vec<Vec<f32>> = vec![
            vec![0.0, 3.9, 6.44],
            vec![3.9, 0.0, 4.7],
            vec![6.44, 4.7, 0.0],
        ];

        let rank = super::calculate_travel_distance(&city_order, &distance_matrix);
        assert_eq!(rank, 15.04);
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
}
