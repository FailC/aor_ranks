use aor_ranks::*;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::Path;

// todo:
// add car names
// write everything to files (done, but sort it into directories?)
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

    let single_leaderboards: HashMap<String, Vec<String>> =
        get_ranked_stages(&stages, &mut players);
    let leaderboard: Vec<String> = get_leaderboard(&mut players);

    // debug part begins here...

    create_folder("./Leaderboards");
    create_folder("./Leaderboards/all_stages");
    let _ = create_file("./Leaderboards", leaderboard, "ranks")
        .map_err(|err| eprintln!("ERROR: failed to create file: {err}"));

    // puts single stages all in one directory
    // split up into country directories?
    create_single_leaderboards(single_leaderboards);

    println!("files created: {:?}", COUNTER);

    Ok(())
}
