mod problems;
use clap::Parser;
use problems::get_problem;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    id: u32,
}

fn main() {
    let cli = Cli::parse();

    match get_problem(cli.id) {
        Some(problem) => {
            println!("Solving Problem {}...", cli.id);
            let solution = problem.solve();
            println!("Solution: {solution}");
        }
        None => {
            eprintln!("Problem {} not implemented yet", cli.id);
            std::process::exit(1);
        }
    }
}
