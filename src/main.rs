use aor_ranks::*;
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // read in user data
    // path : ./target/debug/user_files
    let args: Vec<String> = env::args().collect();

    // for user inpout directory
    if args.len() != 2 {
        panic!("no argument for directory provided");
    }

    let dir = &args[1];
    let dir_path = Path::new(&dir);
    //println!("{:?}", &dir_path);

    //let dir_path = Path::new("./user_files");
    let mut users: Vec<Player> = load_users_from_dir(dir_path)?;
    let _all_stages = load_all_stages();

    let mut stage_vectors: HashMap<String, Vec<Stage>> = build_stage_vectors(&users);
    //dbg!(&stage_vectors);

    rank_stages(&mut stage_vectors, &mut users);

    build_leaderboard(&mut users);
    //    for (stage, vec) in stage_vectors {
    //       println!("{:?} , {:?}", stage, vec);
    //  }

    // Vec<Player>
    // Player {name, HashMap<String, Stage>}
    // String: Stage_name
    // Stage {stage_name, time, car}

    //println!("{}: {:?}", user.name, user.stages);
    // failx: {"Finland_Stage_1_Forward_Dry_60s": Stage { name: "Finland_Stage_1_Forward_Dry_60s", time: 85482, car: 0 }}
    // maybe only use HashMap<stage_name, stage {time, car}> / or just a vec like Player: vec<Stage>

    Ok(())
}
