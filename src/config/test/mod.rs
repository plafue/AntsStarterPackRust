use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use super::*;

#[test]
fn read_config_works() {
    let path = Path::new("test/resources/ant_config");
    let file = File::open(&path).unwrap();
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
