use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::alloc;
use regex::Regex;


fn main() -> Result<(), Error> {
    let path = "day1/input-test-part-1.txt";
    // let path = "day1/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let re = Regex::new(r"[0-9]{1}").unwrap();
    let hay = "aasd23ss111b227c";

    let mat = re.find(hay).unwrap();

    let mut suma: i32 = 0;

    for line in buffered.lines() {
        println!("{:?}", line);

        let cadena = line.unwrap(); 
        
        let mut numeros: Vec<&str> = re.find_iter(&cadena).map(|m| m.as_str()).collect();
        
        println!("{:?}", numeros);


    // println!("Firt element {:?}: ", numeros.pop());

        let primer_caracter = numeros[0].parse::<i32>();
        let ultim_caracter = numeros[numeros.len()-1].parse::<i32>();

        let mut combi = [numeros[0],numeros[numeros.len()-1]].join("").parse::<i32>();

//        println!("{:?}", primer_caracter);
 //       println!("{:?}", ultim_caracter);
  //      println!("{:?}", combi);

        suma = &suma + &combi.unwrap();
    }
    println!("{:?}", suma);

    //    hay.find(&re)
//    for line in buffered.lines() {
//        let cadena = line?;
//        println!("{}", cadena);
//        
        // let first_digit = cadena.find(&re);
//    }
let re = Regex::new(r"[0-9]{1}|eight|one|two|three|four|five|six|seven|nine").unwrap();

let mut sum_calibration: i32 = 0;
let path = "day1/input-test-part-2.txt";
//let path = "day1/input.txt";

let input = File::open(path)?;
let buffered = BufReader::new(input);

fn change_number(numero: &str) -> &str {
    if numero == "one" { "1" }
    else if numero == "two" { "2" }
    else if numero == "three" { "3" }
    else if numero == "four" { "4" }
    else if numero == "five" { "5" }
    else if numero == "six" { "6" }
    else if numero == "seven" { "7" }
    else if numero == "eight" { "8" }
    else if numero == "nine" { "9" }
    else { numero }
}

for line in buffered.lines() {
    println!("{:?}", line);

    // let cadena = line.unwrap(); 
    
    let mut cadena_buscar: String = line.unwrap();

    let mut numeros = Vec::new();

    

    loop {
        println!("1st Buscar {:?}", cadena_buscar);
        let element = re.find(&cadena_buscar).unwrap();

        if ! element.is_empty() { 
            println!("Element trobat {:?}", element.as_str());
            numeros.push(element.as_str());

            let inici = element.start()+1;

            /*
            string té un problema perquè fa servir punters com C, i per tant no "matxaca" la posició
            de memòria. 
            */
            let _cadena_nova: String = cadena_buscar.drain(inici..).collect();
            

            //println!("Nova {:?}", cadena_nova);
            //let cadena_buscar = &cadena_nova.to_string();
            println!("Buscar {:?}", cadena_buscar);
           
        } else {
            break;
        }
    }

    // let mut numeros: Vec<&str> = re.find_iter(&cadena).map(|m| m.as_str()).collect();
    
    println!("{:?}", numeros);

    let primer_caracter = change_number(numeros[0]);
    let ultim_caracter = change_number(numeros[numeros.len()-1]);

    let mut combi = [primer_caracter,ultim_caracter].join("").parse::<i32>();

    println!("{:?}", primer_caracter);
    println!("{:?}", ultim_caracter);
    println!("{:?}", combi);

    sum_calibration = &sum_calibration + &combi.unwrap();
}

println!("{:?}", sum_calibration);


    Ok(())

}