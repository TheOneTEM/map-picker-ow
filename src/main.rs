pub mod gamemode;
pub mod overwatchmap;
use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;
use rand::seq::SliceRandom;
use crate::{gamemode::GameMode, overwatchmap::OWMap};


fn read_lines_to_vec(filename: &str) -> Result<Vec<OWMap>, ()> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
 
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    // removing the first element becuase csv title
    lines.remove(0);
    Ok(
        lines.iter()
            .map(|s| 
                s.split(",")
                .map(|s|
                    s.to_owned()
                )
                .collect()
            )
            .map(|v|
                OWMap::from_vec_string(&v)
            )
            .collect()
    )
    
}
#[derive(Parser, Debug)]
struct Options {
    input_file: String, 
}

fn main() {
    let args = Options::parse();

    let mut rng = rand::rng();
    let filename = &args.input_file;
    let mut lines = read_lines_to_vec(filename).unwrap();
    lines.shuffle(&mut rng);
    let mut all_available_maps: Vec<OWMap> = lines[..9].to_vec();

    let mut last_gamemode: Option<GameMode> = None;
    let mut round_counter = 0;
    loop {
        round_counter += 1;

        // filter currently pickable maps
        let mut currently_available_maps = all_available_maps.clone();
        if let Some(ref mode) = last_gamemode {
            currently_available_maps.retain(|m| m.game_mode != *mode);
        }

        if currently_available_maps.is_empty() {
            currently_available_maps = all_available_maps.clone()
        }

        // print currently available maps
        let available_maps_text = "Map ".to_owned() + &round_counter.to_string() + ". " + &currently_available_maps.len().to_string() + " maps available: ";
        let separator = "=".to_string();
        let mut separator_line = "".to_string();
        for _ in 0..available_maps_text.len() {
            separator_line += &separator;
        }
        println!("{}", available_maps_text);
        println!("{}", separator_line);


        for (idx, map) in currently_available_maps.iter().enumerate() {
            println!("{idx}: {map}")
        }

        // let user select map
        use std::io::{self, Write};

        loop {
            print!(
                "{}\nSelect map (0-{}) > ",
                separator_line,
                currently_available_maps.len() - 1
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input. Try again.");
                continue;
            }

            let index: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input: please enter a number.");
                    continue;
                }
            };

            if index >= currently_available_maps.len() {
                println!("Out of bounds. Please enter a number in range.");
                continue;
            }

            let picked_map = currently_available_maps.remove(index);

            all_available_maps.retain(|m| *m != picked_map);
            last_gamemode = Some(picked_map.game_mode);

            println!("{separator_line}\n");
            break;
        }
    }
}