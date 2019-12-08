extern crate getopts;

use std::env;
use getopts::Options;

use crate::greedy;
use crate::Neighborhood;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

pub fn get_opts() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print help");
    opts.optopt("i",
        "instance",
        "Instance to solve without file end.\n
        Needs to be in instances folder.\n 
        If not specified, all instances are run.",
        ""
    );
    opts.optopt("s", "solver", "Solver as follows\n
        greedy-{beta}\n
        pilot-{beta}\n
        sa-{alpha}-{start-T}{end-T}\n
        local-{iterationlimit}", 
        ""
    );
    opts.optopt("n", "neighborhood", "Neighborhoods: {n1}, {n2}, ... (e.g. te-10, df)\n
        te-{max_block_length} - e.g. te-10\n
        de-{max_block_length} - e.g. de-10\n
        df", ""
    );
    opts.optopt("r", "runs", "Number of runs.", "");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("None?");
            print_usage(&program, opts);
            panic!(f.to_string());
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }



    let runs = match matches.opt_str("runs") {
        None => 1,
        Some(x) => x.parse().unwrap()
    };

    let neighborhood_string = matches.opt_str("neighborhood");
    let solver_string = match matches.opt_str("solver") {
        None => String::from("greedy"),
        Some(x) => x,
    };
    match matches.opt_str("instance") {
        None => select_solver(solver_string, neighborhood_string, None, runs),
        Some(x) => select_solver(solver_string, neighborhood_string, Some(&x[..]), runs)
    };
}

fn select_solver(solver_string: String, neighborhood_string: Option<String>, instance_name: Option<&str>, runs: usize) {
    let solver: Vec<&str> = solver_string.trim().split("-").map(|x| x.trim()).collect();

    match solver[0] {
        "greedy" => {
            let candidate_size = if solver.len() > 1 && !solver[1].is_empty() {
                solver[1].parse().expect("Beta has to be an unsigned integer > 0")
            } else {
                1
            };
            greedy(instance_name, candidate_size, runs);
        },
        _ => {
            println!("Invalid solver selected, run with -h to see the available solvers.");
            return;
        }
    };
}

fn get_neighborhoods(neighborhood_string: String) -> Vec<Neighborhood> {
    neighborhood_string.trim().split(",").map(|x| get_neighborhood(x.trim())).collect()
}

fn get_neighborhood(name: &str) -> Neighborhood {
    let args: Vec<&str> = name.trim().split("-").map(|x| x.trim()).collect();
    let max_block_length = if args.len() > 1 && !args[1].is_empty() {
        Some(args[1].parse().expect("Maximum block length has to be an unsigned integer > 0"))
    } else {
        None
    };

    match args[0] {
        "te" => Neighborhood::TripleEdgeExchange(max_block_length),
        "de" => Neighborhood::DoubleEdgeExchange(max_block_length),
        "df" => Neighborhood::DriverFlip,
        "comp" => Neighborhood::Compound(max_block_length),
        _ => {
            println!("Invalid neighborhood selected, run with -h to see the available neighborhoods.");
            unimplemented!();
        }
    }
}

// fn all_from_env( ) {
//     let args: Vec<String> = env::args().collect();
//     assert!(args.len() >= 2);
//     match &args[1][..] {
//         "greedy" => greedy(None, args[2].parse::<usize>().unwrap(), 1),
//         "pilot" => pilot(None, args[2].parse::<usize>().unwrap(), 1),
//         "local" => {
//             let max_length = if args.len() > 2 {
//                 Some(args[2].parse::<usize>().unwrap())
//             } else {
//                 None
//             };
//             test_all_local_searches(None, max_length)
//         },
//         "grasp" => {
//             let max_length = if args.len() > 2 {
//                 Some(args[2].parse::<usize>().unwrap())
//             } else {
//                 None
//             };
//             grasp(None, 5, Neighborhood::Compound(max_length), StepFunction::BestImprovement, 100, 20000, 5)
//         },
//         "vnd" => {
//             let max_length = if args.len() > 2 {
//                 Some(args[2].parse::<usize>().unwrap())
//             } else {
//                 None
//             };
//             variable_neighborhood(None, vec![
//                 Neighborhood::DoubleEdgeExchange(max_length), 
//                 Neighborhood::DriverFlip, 
//                 Neighborhood::TripleEdgeExchange(max_length)
//             ], 1)
//         },
//         "sa" => {
//             let max_length = if args.len() > 2 {
//                 Some(args[2].parse::<usize>().unwrap())
//             } else {
//                 None
//             };
//             simulated_annealing(None, Neighborhood::Compound(max_length), 5)
//         },
//         _ => unimplemented!()
//     };
// }