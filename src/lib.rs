use std::collections::HashMap;
use std::fs::{self};
use std::io;
use std::path::Path;
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Finland {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Sardinia {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Japan {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Norway {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Germany {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Kenya {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Indonesia {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}
#[allow(dead_code)]
enum Australia {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
    Stage6,
}

impl Finland {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("noormarku"),
            2 => Some("lamppi"),
            3 => Some("palus"),
            4 => Some("lassila"),
            5 => Some("kairila"),
            6 => Some("haaparjarvi"),
            _ => None,
        }
    }
}
impl Sardinia {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("villacidro"),
            2 => Some("san gavino monreale"),
            3 => Some("san benedetto"),
            4 => Some("gennamari"),
            5 => Some("portu maga"),
            6 => Some("montevecchio"),
            _ => None,
        }
    }
}
impl Japan {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("nasu Highland"),
            2 => Some("mount Asama"),
            3 => Some("mount Akagi"),
            4 => Some("nikko"),
            5 => Some("tsumagoi"),
            6 => Some("mount Haruna"),
            _ => None,
        }
    }
}
impl Norway {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("laupstad"),
            2 => Some("vestpollen"),
            3 => Some("stronstad"),
            4 => Some("kvannkjosen"),
            5 => Some("grunnfor"),
            6 => Some("lake Rostavatn"),
            _ => None,
        }
    }
}
impl Germany {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("hockweiler"),
            2 => Some("franzenheim"),
            3 => Some("holzerath"),
            4 => Some("farschweiler"),
            5 => Some("mertesdorf"),
            6 => Some("gonnesweiler"),
            _ => None,
        }
    }
}
impl Kenya {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("mount kenya"),
            2 => Some("karura"),
            3 => Some("homa bay"),
            4 => Some("ndere island"),
            5 => Some("lake baringo"),
            6 => Some("lake nakuru"),
            _ => None,
        }
    }
}
impl Indonesia {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("mount kawi"),
            2 => Some("semangka island"),
            3 => Some("satonda island"),
            4 => Some("oreng valley"),
            5 => Some("sangeang island"),
            6 => Some("kalabakan island"),
            _ => None,
        }
    }
}
impl Australia {
    fn from_number(stage_number: u8) -> Option<&'static str> {
        match stage_number {
            1 => Some("gum scrub"),
            2 => Some("toorooka"),
            3 => Some("nulla nulla"),
            4 => Some("comara canyon"),
            5 => Some("lake lucernia"),
            6 => Some("wombamurra"),
            _ => None,
        }
    }
}

enum Location {
    Finland,
    Sardinia,
    Japan,
    Norway,
    Germany,
    Kenya,
    Indonesia,
    Australia,
}

impl Location {
    fn from_str(location: &str) -> Option<Location> {
        match location {
            "Finland" => Some(Location::Finland),
            "Sardinia" => Some(Location::Sardinia),
            "Japan" => Some(Location::Japan),
            "Norway" => Some(Location::Norway),
            "Germany" => Some(Location::Germany),
            "kenya" => Some(Location::Kenya),
            "Indonesia" => Some(Location::Indonesia),
            "Australia" => Some(Location::Australia),
            _ => None,
        }
    }
}
// Define similar enums for other countries...

#[derive(Debug, Clone)]
pub struct Stage {
    pub name: String,
    pub time: u32,
    pub car: u8,
    pub player_name: String,
    location: String,
    stage_number: u8,
    stage_name: String,
    direction: String,
    weather: String,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub rankings: HashMap<String, u64>,
    pub stages: HashMap<String, Stage>,
}

impl Player {
    fn new(name: String, stages: HashMap<String, Stage>) -> Player {
        Player {
            name,
            score: 0,
            rankings: HashMap::new(),
            stages,
        }
    }
    pub fn get_average_score(&self) -> String {
        let mut score: u64 = 0;
        let mut len = 0;
        for (_h, v) in &self.rankings {
            score += v;
            len += 1;
        }
        format!("average score {}: {score}", self.name, score = score / len)
    }
}

impl Stage {
    pub fn from_line(line: &str, player_name: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        // check for daily, weekly events / for now filter out bonus cars / include Australia DLC?
        if parts.len() == 3
            && !parts[0].contains("Custom")
            && !parts[0].contains("Bonus")
            && !(parts[0].contains("daily") || parts[0].contains("weekly"))
        {
            let name = parts[0].to_string();
            let time = parts[1].parse().ok()?;
            let car = parts[2].parse().ok()?;

            let location_parts: Vec<&str> = parts[0].split("_").collect();
            let location = location_parts[0];
            let stage_number: u8 = location_parts[2].parse().ok()?;
            let location_enum = Location::from_str(location)?;
            let stage_name = match location_enum {
                Location::Finland => Finland::from_number(stage_number),
                Location::Sardinia => Sardinia::from_number(stage_number),
                Location::Japan => Japan::from_number(stage_number),
                Location::Norway => Norway::from_number(stage_number),
                Location::Germany => Germany::from_number(stage_number),
                Location::Kenya => Kenya::from_number(stage_number),
                Location::Indonesia => Indonesia::from_number(stage_number),
                Location::Australia => Australia::from_number(stage_number),
            }?;
            let direction = location_parts[3].to_string();
            let weather = location_parts[4].to_string();

            Some(Stage {
                name,
                time,
                car,
                player_name: player_name.to_string(),
                location: location.to_string(),
                stage_number,
                stage_name: stage_name.to_string(),
                direction,
                weather,
            })
        } else {
            None
        }
    }
    pub fn time_to_string(&self) -> String {
        let total_seconds = self.time / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let milliseconds = self.time % 1000;
        let string = format!("{minutes:02}:{seconds:02}:{milliseconds:03}");
        string
    }
}

pub fn read_stages_from_file(path: &Path, player_name: &str) -> io::Result<HashMap<String, Stage>> {
    let mut stages = HashMap::new();
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if let Some(stage) = Stage::from_line(line, player_name) {
            stages.insert(stage.name.clone(), stage);
        }
    }
    Ok(stages)
}

pub fn load_users_from_dir(dir: &Path) -> io::Result<Vec<Player>> {
    let mut players: Vec<Player> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            // player_name
            let player_name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string();

            if let Ok(stages) = read_stages_from_file(&path, &player_name) {
                players.push(Player::new(player_name, stages))
            }
        }
    }
    Ok(players)
}

// best rust code ever
pub fn _load_all_stages() -> Vec<String> {
    let contents = fs::read_to_string("Leaderboards.txt").unwrap();
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    let mut stage_names: Vec<String> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        stage_names.push(parts[0].to_owned());
    }
    stage_names
}

pub fn collect_stages_from_players(players: &[Player]) -> HashMap<String, Vec<Stage>> {
    let mut every_stage: HashMap<String, Vec<Stage>> = HashMap::new();

    for player in players {
        for stage in player.stages.values() {
            every_stage
                .entry(stage.name.clone())
                .or_insert_with(Vec::new)
                .push(stage.clone());
        }
    }
    every_stage
}

// should only return or mutate stuff, in main generate all files / directories
pub fn rank_stages(
    every_stage: &HashMap<String, Vec<Stage>>,
    players: &mut Vec<Player>,
) -> HashMap<String, Vec<String>> {
    let mut single: HashMap<String, Vec<String>> = HashMap::new();

    for (stage_name, stages) in every_stage.iter() {
        if let Some(fastest_stage) = stages.first() {
            let fastest_time = fastest_stage.time as f64;
            // update player scores
            for (pos, stage) in stages.iter().enumerate() {
                let rank: i32 = pos as i32 + 1;
                let player_time = stage.time as f64;
                // scoring algorithm
                let score =
                    1000.0 * (fastest_time / player_time) * 0.988f64.powf(rank as f64 - 1.0);
                // Find player and update score
                if let Some(player) = players.iter_mut().find(|p| p.name == stage.player_name) {
                    player.score += score as u32;
                    player
                        .rankings
                        .entry(stage_name.clone())
                        .or_insert(score as u64);

                    let string = format!(
                        "{} {} {} {}",
                        player.name,
                        stage.time_to_string(),
                        score as i64,
                        stage.car
                    );
                    let n = format!("{}: {}", stage.location, stage.stage_name);
                    single.entry(n).or_insert(Vec::new()).push(string);
                }
            }
        }
    }
    single
}

pub fn build_leaderboard(players: &mut Vec<Player>) -> Vec<String> {
    // Sort players by score in descending order
    players.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut string: Vec<String> = Vec::new();
    for (rank, player) in players.iter().enumerate() {
        let s = format!("{}: {} \t{:.2} points", rank + 1, player.name, player.score);
        string.push(s);
    }
    string
}

//pub fn convert_ms_to_string(ms: u32) -> String {
//    let total_seconds = ms / 1000;
//    let minutes = total_seconds / 60;
//    let seconds = total_seconds % 60;
//    let milliseconds = ms % 1000;
//    let string = format!("{minutes:02}:{seconds:02}:{milliseconds:03}");
//    string
//}
