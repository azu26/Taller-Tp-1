use crate::coordenada::Coordenada;
use crate::piezas;
use crate::ErroresAjedrez;

use piezas::alfil::Alfil;
use piezas::caballo::Caballo;
use piezas::dama::Dama;
use piezas::peon::Color;
use piezas::peon::Peon;
use piezas::puede_comer::PuedeComer;
use piezas::rey::Rey;
use piezas::torre::Torre;

const EMPATE: &str = "E";
const NEGRAS_GANAN: &str = "N";
const BLANCAS_GANAN: &str = "B";
const NINGUNA_CAPTURA: &str = "P";
const DAMA_NEGRA: char = 'D';
const DAMA_BLANCA: char = 'd';
const REY_NEGRO: char = 'R';
const REY_BLANCO: char = 'r';
const TORRE_NEGRA: char = 'T';
const TORRE_BLANCA: char = 't';
const ALFIL_NEGRO: char = 'A';
const ALFIL_BLANCO: char = 'a';
const PEON_NEGRO: char = 'P';
const PEON_BLANCO: char = 'p';
const CABALLO_NEGRO: char = 'C';
const CABALLO_BLANCO: char = 'c';
const ESPACIO: char = ' ';
const SALTO_DE_LINEA: char = '\n';
const POSICION_VACIA: char = '_';

pub fn inicializar_pieza(
    letra: char,
    coordenada: Coordenada,
) -> Result<Box<dyn PuedeComer>, ErroresAjedrez> {
    match letra {
        DAMA_NEGRA | DAMA_BLANCA => Ok(Box::new(Dama::new(coordenada))),
        REY_NEGRO | REY_BLANCO => Ok(Box::new(Rey::new(coordenada))),
        TORRE_NEGRA | TORRE_BLANCA => Ok(Box::new(Torre::new(coordenada))),
        ALFIL_NEGRO | ALFIL_BLANCO => Ok(Box::new(Alfil::new(coordenada))),
        CABALLO_NEGRO | CABALLO_BLANCO => Ok(Box::new(Caballo::new(coordenada))),
        PEON_NEGRO => Ok(Box::new(Peon::new(Color::Negro, coordenada))),
        PEON_BLANCO => Ok(Box::new(Peon::new(Color::Blanco, coordenada))),
        _ => Err(ErroresAjedrez::Pieza),
    }
}

pub fn resultado_jugadas(
    piezas: Vec<char>,
    posiciones: Vec<usize>,
) -> Result<&'static str, ErroresAjedrez> {
    let pieza1 = inicializar_pieza(piezas[0], Coordenada::new(posiciones[0]))?;
    let pieza2 = inicializar_pieza(piezas[1], Coordenada::new(posiciones[1]))?;

    let negras_comen = pieza1.puede_comer(Coordenada::new(posiciones[1]));
    let blancas_comen = pieza2.puede_comer(Coordenada::new(posiciones[0]));

    if negras_comen && blancas_comen {
        return Ok(EMPATE);
    } else if negras_comen {
        return Ok(NEGRAS_GANAN);
    } else if blancas_comen {
        return Ok(BLANCAS_GANAN);
    }
    Ok(NINGUNA_CAPTURA)
}

pub fn validar_tablero(table: String) -> Result<(Vec<char>, Vec<usize>), ErroresAjedrez> {
    let mut piezas = Vec::new();
    let mut posiciones = Vec::new();

    for (i, c) in table.chars().enumerate() {
        if (c == ESPACIO && i % 2 == 0) || (c == POSICION_VACIA && i % 2 != 0) || i > 128 {
            return Err(ErroresAjedrez::Tablero);
        }
        if c != SALTO_DE_LINEA && c != ESPACIO && c != POSICION_VACIA {
            if i % 2 != 0 {
                return Err(ErroresAjedrez::Tablero);
            }
            piezas.push(c);
            posiciones.push(i);
        }
    }
    Ok((piezas, posiciones))
}
