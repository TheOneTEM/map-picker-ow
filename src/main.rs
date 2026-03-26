use core::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::SliceRandom;

enum GameMode {
    Assault,
    Payload,
    Hybrid,
    Flashpoint, 
    Push,
    Clash,
}
impl GameMode {
    fn from_string(s: String) -> Option<GameMode> {
        match s.to_ascii_lowercase() {
            val if val == "assault" => Some(Self::Assault),
            val if val == "payload" => Some(Self::Payload),
            val if val == "hybrid" => Some(Self::Hybrid),
            val if val == "flashpoint" => Some(Self::Flashpoint),
            val if val == "push" => Some(Self::Push),
            val if val == "clash" => Some(Self::Clash),
            _ => None
        }
    }
    #[inline]
    fn is_sided(self) -> bool {
        matches!(self, Self::Assault | Self::Payload | Self::Hybrid)
    }
    fn to_string(&self) -> String {
        match self {
            Self::Assault => "assault",
            Self::Payload => "payload",
            Self::Hybrid => "hybrid",
            Self::Flashpoint => "flashpoint",
            Self::Push => "push",
            Self::Clash => "clash"

        }.to_string()
    }
}


fn read_lines_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn main() -> io::Result<()> {
    let mut rng = rand::rng();
    let filename = "maps.txt";

    let mut lines = read_lines_to_vec(filename)?;
    lines.shuffle(&mut rng);
    let available_maps_text = "Available maps for this round: ".to_string();
    let separator = "=".to_string();
    let mut separator_line = "".to_string();
    for _ in 0..available_maps_text.len() {
        separator_line += &separator;
    }
    println!("{}", available_maps_text);
    println!("{}", separator_line);
    for (i, line) in lines.iter().enumerate() {
        if i < 9 {
            println!("{}", line);
        }
    }

    Ok(())
}