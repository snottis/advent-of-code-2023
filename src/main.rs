mod api;
mod cli;
mod tasks;

fn main() {
    let matches = cli::get_matches();

    match matches.subcommand() {
        Some(("fetch", fetch_matches)) => {
            let day = (*fetch_matches.get_one::<String>("day").unwrap())
                .parse::<u16>()
                .unwrap();
            api::fetch_input(day);
        }
        Some(("solve", solve_matches)) => {
            let day = (*solve_matches.get_one::<String>("day").unwrap())
                .parse::<u16>()
                .unwrap();
            let level = (*solve_matches.get_one::<String>("level").unwrap())
                .parse::<u8>()
                .unwrap();
            tasks::solve(day, level);
        }
        Some(("submit", submit_matches)) => {
            let day = (*submit_matches.get_one::<String>("day").unwrap())
                .parse::<u16>()
                .unwrap();
            let level = (*submit_matches.get_one::<String>("level").unwrap())
                .parse::<u8>()
                .unwrap();
            api::submit_answer(level, day, tasks::solve(day, level))
        }
        _ => unreachable!(),
    }
}
