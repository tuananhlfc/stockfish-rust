// use std::error::Error;
use std::{collections::HashMap, io};

use crate::position::Position;
use crate::types::*;

struct UciOption {
    value_type: String,
    default_val: String,
    current_val: String,
    min_val: i32,
    max_val: i32,
    idx: usize,
}

impl From<(&str, usize)> for UciOption {
    fn from((v, idx): (&str, usize)) -> Self {
        return UciOption {
            value_type: "string".to_string(),
            default_val: v.to_string(),
            current_val: v.to_string(),
            min_val: 0,
            max_val: 0,
            idx: idx,
        };
    }
}

impl From<(bool, usize)> for UciOption {
    fn from((check, idx): (bool, usize)) -> Self {
        let v = match check {
            true => "true",
            false => "false",
        };
        return UciOption {
            value_type: "check".to_string(),
            default_val: v.to_string(),
            current_val: v.to_string(),
            min_val: 0,
            max_val: 0,
            idx: idx,
        };
    }
}

// button
impl From<usize> for UciOption {
    fn from(idx: usize) -> Self {
        return UciOption {
            value_type: "button".to_string(),
            default_val: "".to_string(),
            current_val: "".to_string(),
            min_val: 0,
            max_val: 0,
            idx: idx,
        };
    }
}

// spin
impl From<(f64, i32, i32, usize)> for UciOption {
    fn from((v, minv, maxv, idx): (f64, i32, i32, usize)) -> Self {
        return UciOption {
            value_type: "spin".to_string(),
            default_val: v.to_string(),
            current_val: v.to_string(),
            min_val: minv,
            max_val: maxv,
            idx: idx,
        };
    }
}

// combo
impl From<(&str, &str, usize)> for UciOption {
    fn from((v, cur, idx): (&str, &str, usize)) -> Self {
        return UciOption {
            value_type: "combo".to_string(),
            default_val: v.to_string(),
            current_val: cur.to_string(),
            min_val: 0,
            max_val: 0,
            idx: idx,
        };
    }
}

fn uci_init<'a>() -> HashMap<&'a str, UciOption> {
    let mut m = HashMap::new();
    m.insert("Debug Log File", UciOption::from(("", 0)));
    m.insert("Threads", UciOption::from((1.0, 1, 1024, 1)));
    m.insert("Hash", UciOption::from((16.0, 1, 33554432, 2)));
    m.insert("Clear Hash", UciOption::from(3));
    m.insert("Ponder", UciOption::from((false, 4)));
    m.insert("MultiPV", UciOption::from((1.0, 1, 500, 5)));
    m.insert("Skill Level", UciOption::from((20.0, 0, 20, 6)));
    m.insert("Move Overhead", UciOption::from((10.0, 0, 5000, 7)));
    m.insert("Slow Mover", UciOption::from((100.0, 10, 1000, 8)));
    m.insert("nodestime", UciOption::from((0.0, 0, 10000, 9)));
    m.insert("UCI_Chess960", UciOption::from((false, 10)));
    m.insert("UCI_AnalyseMode", UciOption::from((false, 11)));
    m.insert("UCI_LimitStrength", UciOption::from((false, 12)));
    m.insert("UCI_Elo", UciOption::from((3190.0, 1320, 3190, 13)));
    m.insert("UCI_ShowWDL", UciOption::from((false, 14)));
    m.insert("SyzygyPath", UciOption::from(("<empty>", 15)));
    m.insert("SyzygyProbeDepth", UciOption::from((1.0, 1, 100, 16)));
    m.insert("Syzygy50MoveRule", UciOption::from((true, 17)));
    m.insert("SyzygyProbeLimit", UciOption::from((7.0, 0, 7, 18)));
    m.insert("Use NNUE", UciOption::from((true, 19)));
    m.insert("EvalFile", UciOption::from(("nn-e1fb1ade4432.nnue", 20)));

    m
}

pub fn uci_loop() {
    let options_map = uci_init();
    let mut position = Position::new();
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let input = line.trim();
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let cmd = tokens[0];

        match cmd {
            "quit" | "stop" => break,
            "ponderhit" => (),
            "uci" => handle_uci(&options_map),
            "setoption" => setoption(input),
            "go" => handle_go(),
            "position" => handle_position(&mut position, tokens),
            "ucinewgame" => (),
            "isready" => handle_is_ready(),
            _ => continue,
        }
    }
}

fn handle_uci(options_map: &HashMap<&str, UciOption>) {
    println!("id name stockfish-rust-anhphan");
    for i in 0..options_map.len() {
        for (k, v) in options_map.iter() {
            if v.idx == i {
                let (value_type, default_val) = (&v.value_type, &v.default_val);
                print!("option name {k} type {value_type}");
                if value_type == "string" || value_type == "check" || value_type == "combo" {
                    println!(" default {default_val}")
                } else if value_type == "spin" {
                    let (min_val, max_val) = (&v.min_val, &v.max_val);
                    println!(" default {default_val} min {min_val} max {max_val}")
                }
            }
        }
    }
    println!("uciok");
}

fn handle_is_ready() {
    println!("readyok");
}

fn setoption(stream: &str) {
    println!("{stream}");
}

fn handle_position(position: &mut Position, tokens: Vec<&str>) {
    // this function could be refactor
    if tokens.len() < 2 {
        return;
    }
    let fen = match tokens[1].to_lowercase().as_str() {
        "startpos" => START_POS_FEN.to_string(),
        "fen" => get_fen_str(&tokens[2..]),
        _ => return,
    };

    position.set_position(fen.as_str());
    let moves_idx = tokens.iter().position(|&x| x == "moves");
    if let Some(idx) = moves_idx {
        for m in tokens[(idx + 1)..].iter() {
            position.do_move(m);
        }
    }
}

fn get_fen_str(words: &[&str]) -> String {
    let mut fen: String = Default::default();
    for w in words.iter() {
        if *w == "moves" {
            break;
        }
        fen.push_str(w);
        fen.push_str(" ");
    }
    fen
}

fn handle_go() {
    // start counting time
}
