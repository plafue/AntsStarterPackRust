use std::io::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
mod config;
mod game;
use game::*;
use config::*;

#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let config = read_config(&mut BufReader::new(stdin.lock()));
    let mut board = Board::new();

    let debug_log_path = Path::new("E:\\rustbot.log");
    let mut options = OpenOptions::new();
    options
        .write(true)
        .create(true);
    let debug_log_file = options.open(&debug_log_path).unwrap();
    let mut log_writer = BufWriter::new(debug_log_file);
    log_writer.write_all(format!("Start game: Config: {:?}\n", config).as_bytes());
    println!("go");

    run_game(&mut log_writer, &mut BufReader::new(stdin.lock()), &mut BufWriter::new(stdout.lock()), config, &mut board);
}

fn run_game<L: Write, R: Read, W: Write>(log_writer: &mut BufWriter<L>,in_stream: &mut BufReader<R>, out_stream: &mut BufWriter<W>, config: GameConfig, board: &mut Board) {
    loop {
        out_stream.flush();
        log_writer.flush();
        let mut line = String::new();
        in_stream.read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.is_empty() {
            continue;
        } else if line.eq("go") {
            out_stream.write_all(b"go\n");
        } else {
            log_writer.write_all(format!("RECEIVED: {}\n", line).as_bytes());
            continue;
        }
    }

}
