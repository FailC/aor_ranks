use aor_ranks::*;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::Path;

pub mod game;
// TODO:
// add car names
// calculating average position too, average score done

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // default path:
    //let dir_path = Path::new("./user_files");
    if args.len() != 2 {
        eprintln!("no argument for directory provided");
        eprintln!("example: cargo run -- ./testfiles");
        return Ok(());
    }

    let dir = &args[1];
    let dir_path = Path::new(&dir);

    print!("loading files..");
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut players: Vec<Player> = load_users_from_dir(dir_path)?;
    println!("{} players", players.len());

    let stages: HashMap<String, Vec<Stage>> = collect_stages_from_players(&players);

    create_folder("./Leaderboards");
    create_folder("./Leaderboards/all_stages");
    create_folder("./Leaderboards/groups");
    print!("generating files..");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let single_leaderboards: HashMap<String, Vec<String>> =
        get_ranked_stages(&stages, &mut players);
    let leaderboard: Vec<String> = get_leaderboard(&mut players);

    create_group_leaderboards(&players);
    std::io::stdout().flush().expect("Failed to flush stdout");
    let _ = create_file("./Leaderboards", leaderboard, "ranks")
        .map_err(|err| eprintln!("ERROR: failed to create file: {err}"));

    // puts single stages all in one directory
    // split up into country directories?
    create_single_leaderboards(&single_leaderboards);

    // println!("files created: {:?}", COUNTER);
    println!("done");
    Ok(())
}

// 200 leaderboard files roughly 5 sec runtime (debug) entire program, (Ryzen 5 3600)
// 1.6s with release build (500 files ~5sec)
#[cfg(test)]
mod tests {
    use crate::game;
    use std::time::Instant;

    #[test]
    // changed to static variable instead of running get_locations() every time a Stage gets
    // created
    fn test_get_locations_timing() {
        let count = 500;
        let start_time = Instant::now();
        for _ in 0..count {
            let _locations = game::locations::get_locations();
        }
        let duration = start_time.elapsed();

        // cargo test -- --show-output
        println!(
            "Time taken to execute get_locations {} times: {:?}",
            count, duration
        );
    }
}
