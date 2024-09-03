use aor_ranks::*;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::Path;

// todo:
// add average map score
// add car names, stage names (for single leaderboards)
// one Leaderboard for every stage?
// write everything to files

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

    let mut stages: HashMap<String, Vec<Stage>> = collect_stages_from_players(&players);

    for x in stages.values_mut() {
        x.sort_by(|a, b| a.time.cmp(&b.time));
    }

    let _single_leaderboards = rank_stages(&stages, &mut players);

    let _leaderboard = build_leaderboard(&mut players);

    let p = &players[0];
    dbg!(p);

    //for e in leaderboard {
    //    println!("{}", e);
    //}

    //for p in players {
    //    p.average_score()
    //}

    Ok(())
}
