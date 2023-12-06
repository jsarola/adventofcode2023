use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;
use std::collections::HashSet;

// const MAX_ROW: usize = 140;
// const MAX_COLUMN: usize = 140;
// const PATH: &str = "day3/input.txt";

// const PATH: &str = "day4/input-test-part-1.txt";
const PATH: &str = "day4/input.txt";

pub fn exec_day4() -> Result<(), Error> {
    println!("Hello world day 4");

    let input = File::open(PATH)?;
    let buffered = BufReader::new(input);

    let mut resultat: u32 = 0; 

    fn vecs_match (a: &Vec<usize>, b: &Vec<usize>) -> usize {
        let s1: HashSet<_> = a.iter().copied().collect();
        let s2: HashSet<_> = b.iter().copied().collect();

        let diff: Vec<_> = s1.intersection(&s2).collect();
    
        println!("{:?}", diff);
        
        
        diff.len()
    }
    
    for line in buffered.lines() {
    
        println!("{:?}", line);

        let cadena: String = line.unwrap();

        let re = Regex::new(r":").unwrap();
        let tmp: Vec<&str> = re.split(&cadena).collect();
        
        // println!("{:?}", tmp);

        // note: | needs to be scaped with \
        let re = Regex::new(r"\|").unwrap();
        let numeros: Vec<&str> = re.split(&tmp[1].trim()).collect();

        let premiats: Vec<usize> = numeros[0].trim().split(" ").filter_map(|word| usize::from_str(word).ok()).collect();
        let bingo: Vec<usize> = numeros[1].trim().split(" ").filter_map(|word| usize::from_str(word).ok()).collect();

        // println!("els meus numeros {:?} {:?}", premiats, bingo);
        
        let matching = vecs_match(&premiats, &bingo);

        println!("els matchs son {:?}", vecs_match(&premiats, &bingo));

        if matching > 0 {
            let carta = u32::pow(2, (matching - 1).try_into().unwrap());
            resultat += carta;
        }

        println!("i el resultat final és {:?}", resultat);
        
        /*
        1, 2, 4, 8, 16, 32, 64, 128

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
*/
    }

    let a = [1, 2, 3, 5];
    let b = [1, 1, 3, 3, 5, 6];

    let matching = a.iter().zip(&b).filter(|&(a, b)| a == b).count();
    println!("{}", matching);

    Ok(())
}