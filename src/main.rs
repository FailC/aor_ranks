use aor_ranks::*;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // for user input directory
    if args.len() != 2 {
        panic!("no argument for directory provided");
    }

    let dir = &args[1];
    let dir_path = Path::new(&dir);

    print!("loading files..");
    std::io::stdout().flush().expect("Failed to flush stdout");

    // default path:
    //let dir_path = Path::new("./user_files");
    let mut players: Vec<Player> = load_users_from_dir(dir_path)?;
    println!("{} players", players.len());

    let mut stages: HashMap<String, Vec<Stage>> = collect_stages_from_players(&players);

    //dbg!(&stages);
    rank_stages(&mut stages, &mut players);
    build_leaderboard(&mut players);

    Ok(())
}
