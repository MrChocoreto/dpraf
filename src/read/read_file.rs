use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

pub fn read_file()-> io::Result<()> {
    let data_path: String = String::from("D:\\Download\\README.md");

    // Abre el archivo
    let file = File::open(data_path)?;

    // Crea un BufReader para leer el archivo línea por línea
    let reader = BufReader::new(file);

    // Itera sobre cada línea del archivo
    for line in reader.lines() {
        // Maneja cualquier error que pueda ocurrir al leer una línea
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}