use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;


pub fn exec_day2() -> Result<(), Error> {
    println!("Hello world day 2");
    
    //let path = "day2/input-test-part-1.txt";
    let path = "day2/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let def_blue: usize = 14;
    let def_green: usize = 13;
    let def_red: usize = 12;

    let mut suma: usize = 0;

    let mut suma_power: usize = 0;

    let mut no_es_pot: bool;

    let mut power: usize;

    for line in buffered.lines() {
        no_es_pot = false;

        println!("{:?}", line);

        let cadena: String = line.unwrap();

        let re = Regex::new(r":").unwrap();
        let game: Vec<&str> = re.split(&cadena).collect();
        
        // println!("{:?}", game);

        let re = Regex::new(r"[0-9]+").unwrap();
        let gameid = re.find(game[0]).unwrap().as_str().parse::<usize>();

        println!("game id {:?}", gameid);
        //println!("{:?}", game);
        
        let re = Regex::new(r";").unwrap();
        let tirades: Vec<&str> = re.split(&game[1]).collect();
        println!("tirades {:?}", tirades);

        let mut min_blue: usize = 1;
        let mut min_green: usize = 1;
        let mut min_red: usize = 1;

        for tirada in tirades {
            let re = Regex::new(r",").unwrap();
            let cubs: Vec<&str> = re.split(&tirada).collect();
            println!("cubs {:?}", cubs);

            for cub in cubs {
                let re = Regex::new(r"[0-9]+").unwrap();
                let numcubs = re.find(&cub).unwrap().as_str().parse::<usize>().unwrap();
                let re = Regex::new(r"blue|green|red").unwrap();
                let color = re.find(&cub).unwrap().as_str();

                println!("color + num {:?} - {:?}", color, numcubs);
                match color {
                    "blue" => {
                        if def_blue < numcubs { no_es_pot = true };
                        if min_blue < numcubs { min_blue = numcubs };
                    },
                    "green" => {
                        if def_green < numcubs { no_es_pot = true};
                        if min_green < numcubs { min_green = numcubs };
                    },
                    "red" => {
                        if def_red < numcubs { no_es_pot = true};
                        if min_red < numcubs { min_red = numcubs };
                    },
                    _ => no_es_pot = true,
                }
            }
        }

        if ! no_es_pot {
            suma += gameid.unwrap();
        }

        power = min_blue * min_green * min_red;
        suma_power += power;
        println!("power {:?} ", power);

    }

    println!("Resultat 1 {:?}", suma);
    println!("Resultat 2 {:?}", suma_power);
    Ok(())


}