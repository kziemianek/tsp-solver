fn main() {
    cli::start();
}

mod cli {
    use clap::{App, Arg, ArgMatches};
    use std::cmp::Ordering::Equal;
    use tspsolver::solve;

    pub fn start() {
        let matches = App::new("tsp-solver")
            .version("0.1.0")
            .author("Kasper Ziemianek <kasper.ziemianek@gmail.com>")
            .about("Helps salesman find the shortest route!")
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("FILE")
                    .help("Sets problem instance file")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("duration")
                    .short("d")
                    .long("duration")
                    .value_name("DURATION")
                    .help("Sets computation duration in seconds")
                    .takes_value(true)
                    .default_value(&"5"),
            )
            .arg(
                Arg::with_name("alg")
                    .short("a")
                    .long("algorithm")
                    .value_name("ALGORITHM")
                    .help("Sets meta-heuristics algorithm")
                    .takes_value(true)
                    .default_value(&"hill-climbing"),
            )
            .arg(
                Arg::with_name("runs")
                    .short("r")
                    .long("runs")
                    .value_name("RUNS")
                    .help("Sets number of alrogithm runs")
                    .takes_value(true)
                    .default_value(&"1"),
            )
            .arg(
                Arg::with_name("parallel")
                    .short("p")
                    .long("parallel")
                    .value_name("PARALLEL")
                    .takes_value(false)
                    .help("Sets parallel flag"),
            )
            .get_matches();

        let file = get_file(&matches);
        let computation_duration = get_duration(&matches);
        let alg = get_alg(&matches);
        let runs = get_runs(&matches);
        let parallel = get_parallel(&matches);
        let result = solve(&file, computation_duration, &alg, runs, parallel);

        for (i, result) in result.iter().enumerate() {
            match result {
                Ok(v) => println!("#{} score {}", i + 1, v.travel_distance),
                Err(v) => println!("#{} could not solve problem, error: {}", i + 1, v),
            }
        }

        let mut distances: Vec<f32> = result
            .iter()
            .filter(|result| result.is_ok())
            .map(|result| result.as_ref())
            .map(|result| result.unwrap())
            .map(|result| result.travel_distance)
            .collect();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        match distances.len() > 0 {
            true => println!("Best score {}", distances.get(0).unwrap()),
            false => {}
        }
    }

    fn get_file(matches: &ArgMatches) -> String {
        get_argument(matches, "file")
    }

    fn get_duration(matches: &ArgMatches) -> i64 {
        get_argument(matches, "duration").parse().unwrap()
    }

    fn get_alg(matches: &ArgMatches) -> String {
        get_argument(matches, "alg")
    }

    fn get_runs(matches: &ArgMatches) -> i32 {
        get_argument(&matches, "runs").parse().unwrap()
    }

    fn get_parallel(matches: &ArgMatches) -> bool {
        matches.is_present("parallel")
    }

    fn get_argument(matches: &ArgMatches, name: &str) -> String {
        matches.value_of(name).unwrap().to_owned()
    }
}
