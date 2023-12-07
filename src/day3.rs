use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const MAX_ROW: usize = 10;
const MAX_COLUMN: usize = 10;
const PATH: &str = "day3/input-test-part-1.txt";

// const MAX_ROW: usize = 140;
// const MAX_COLUMN: usize = 140;
// const PATH: &str = "day3/input.txt";


pub fn exec_day3() -> Result<(), Error> {
    println!("Hello world day 2");

    fn comprovar_veins(matrix: &[[char; MAX_ROW]; MAX_COLUMN], fila: usize, ini: usize, fi: usize) -> bool {
        let mut comprovar: bool = false;

        let start: usize;
        let end: usize;

        if ini > 1 { start = ini - 1; } else { start = 0; }
        if fi < MAX_COLUMN - 1 { end = fi + 1; } else { end = MAX_COLUMN - 1; }

        // println!("buscar {:?} {:?}-{:?}", fila, start, end);

        if fila > 0 {
            let row = fila - 1;
            for column in start..end+1 {
                if ! matrix[row][column].is_numeric() {
                    if matrix[row][column] != '.' {
                        comprovar = true;
                    }
                }
            }
        }
        if fila < MAX_ROW - 1 {
            let row = fila + 1;
            for column in start..end+1 {
                if ! matrix[row][column].is_numeric() {
                    if matrix[row][column] != '.' {
                        comprovar = true;
                    }
                }
            }
        }
        if ini > 1 {
            if matrix[fila][start] != '.' {
                comprovar = true;
            }
        }
        if fi < MAX_COLUMN - 1 {
            if matrix[fila][end] != '.' {
                comprovar = true;
            }
        }
        comprovar
    }

    fn buscar_producte(lamatriu: &[[char; MAX_ROW]; MAX_COLUMN], fila: usize, columna: usize) -> usize {
        if lamatriu[fila][columna].is_numeric() {
            lamatriu[fila][columna].to_string().parse::<usize>().unwrap()
        } else {
            0
        }
    }
    
    let input = File::open(PATH)?;
    let buffered = BufReader::new(input);

    let mut matriu:[[char; MAX_ROW]; MAX_COLUMN] = [['.'; MAX_ROW]; MAX_COLUMN];

    let mut row = 0;
    let mut column = 0;
    for line in buffered.lines() {
        let letters: Vec<char> = line.unwrap().chars().collect();
        for element in letters {
            matriu[row][column] = element;
            column += 1;
        }
        column = 0;
        row +=1;
    }

    // println!("{:?}", matriu);

    let mut numero: String = "".to_string();

    let mut suma: u32 = 0;

    let mut trobat_numero: bool = false;

    let mut pos_ini: usize = 0;
    let mut pos_fin: usize = 0; 
    
    // let mut guardats: [(String, usize, usize, usize); 1000] = [("0".to_string(), 0, 0, 0); 1000];
    // let mut control: usize = 0;

    for i in 0..MAX_ROW {
        for j in 0..MAX_COLUMN {
            if matriu[i][j].is_numeric() {
                if trobat_numero {
                    pos_fin = j;

                } else {
                    pos_ini = j;
                    pos_fin = j;
                }
                numero.push(matriu[i][j]);
                trobat_numero = true;
            } else {
                if trobat_numero {
                    
                    if comprovar_veins(&matriu, i, pos_ini, pos_fin) {
                        // println!("trobat {:?} {:?} {:?}-{:?}", numero, i, pos_ini, pos_fin);
                        suma += numero.parse::<u32>().unwrap();

                        let guardar: (String, usize, usize, usize) = (numero.to_string(), i, pos_ini, pos_fin);
                        // guardats[control] = guardar;
                        // control += 1;
                        println!("{:?}", guardar);
                    }
                    trobat_numero = false;
                    numero.clear();
                }
            }
        }
        if trobat_numero {
            if comprovar_veins(&matriu, i, pos_ini, pos_fin) {
                // println!("trobat {:?} {:?} {:?}-{:?}", numero, i, pos_ini, pos_fin);
                suma += numero.parse::<u32>().unwrap();
            }
            trobat_numero = false;
            numero.clear();
        }
        
    }

    println!("Suma part 1 {:?} ", suma);

    for i in 0..MAX_ROW {
        for j in 0..MAX_COLUMN {
            if matriu[i][j] == '*' {
                println!("trobat {:?}-{:?}", i, j);
                let _tmp = buscar_producte(&matriu, i, j);
            }
        }
        
    }

    Ok(())
}