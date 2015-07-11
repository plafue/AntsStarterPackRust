use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameConfig {
    loadtime: i64,
    turntime: i64,
    rows: i64,
    cols: i64,
    turns: i64,
    viewradius2: i64,
    attackradius2: i64,
    spawnradius2: i64,
    player_seed : i64
}

pub fn read_config<R: Read>(reader: &mut BufReader<R>) -> GameConfig {
    let fallback_value: i64 = 0;
    let config_params = read_lines_until_go(reader);

    GameConfig {
        loadtime: config_params.get("loadtime").unwrap_or(&fallback_value).clone(),
        turntime: config_params.get("turntime").unwrap_or(&fallback_value).clone(),
        rows: config_params.get("rows").unwrap_or(&fallback_value).clone(),
        cols: config_params.get("cols").unwrap_or(&fallback_value).clone(),
        turns: config_params.get("turns").unwrap_or(&fallback_value).clone(),
        viewradius2: config_params.get("viewradius2").unwrap_or(&fallback_value).clone(),
        attackradius2: config_params.get("attackradius2").unwrap_or(&fallback_value).clone(),
        spawnradius2: config_params.get("spawnradius2").unwrap_or(&fallback_value).clone(),
        player_seed : config_params.get("player_seed").unwrap_or(&fallback_value).clone()
    }
}

fn read_lines_until_go<R: Read>(reader: &mut BufReader<R>) ->  HashMap<String,i64> {
    let mut config_params: HashMap<String,i64> = HashMap::new();
    let mut done = false;

    while !done {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if !line.is_empty() {
            line = line.trim().to_string();
            if line.eq("go") {
                done = true;
            } else {
                let mut pair = line.trim().split(" ");
                let key = pair.next().unwrap();
                let value = pair.next().unwrap();
                config_params.insert(
                    key.to_string(),
                    value.parse::<i64>().unwrap());
            }
        }
    }
    config_params
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io::BufReader;

    use super::*;
    #[test]
    fn read_config_works() {
        let path = Path::new("test/resources/ant_config");
        let mut file = File::open(&path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let config = read_config(&mut buf_reader);
        assert_eq!(config.loadtime, 1);
        assert_eq!(config.turntime, 2);
        assert_eq!(config.rows, 3);
        assert_eq!(config.cols, 4);
        assert_eq!(config.turns, 5);
        assert_eq!(config.viewradius2, 6);
        assert_eq!(config.attackradius2, 7);
        assert_eq!(config.spawnradius2, 8);
        assert_eq!(config.player_seed , 9);
    }
}
