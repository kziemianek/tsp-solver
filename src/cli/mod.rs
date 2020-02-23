use clap::{App, Arg};

pub fn start<'a, 'b>() -> App<'a, 'b> {
    App::new("tsp-solver")
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
                .help("Sets parallel flag")
                .takes_value(true)
                .default_value(&"false"),
        )
}