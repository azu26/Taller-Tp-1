use std::env;
use std::fs;

mod ajedrez;
mod coordenada;
mod errores;
mod piezas;

use crate::coordenada::Coordenada;
use errores::ErroresAjedrez;

pub fn jugar_ajedrez() -> Result<(), ErroresAjedrez> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let tablero = fs::read_to_string(file_path)?;

    let (piezas, posiciones) = ajedrez::validar_tablero(tablero)?;
    let resultado = ajedrez::resultado_jugadas(piezas, posiciones)?;
    println!("{}", resultado);

    Ok(())
}

fn main() -> Result<(), ErroresAjedrez> {
    if let Err(err) = jugar_ajedrez() {
        println!("Error: {}", err);
    }

    Ok(())
}
