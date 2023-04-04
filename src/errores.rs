use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ErroresAjedrez {
    Pieza,
    Tablero,
    Archivo,
}

impl From<std::io::Error> for ErroresAjedrez {
    fn from(_: std::io::Error) -> ErroresAjedrez {
        ErroresAjedrez::Archivo
    }
}

impl fmt::Display for ErroresAjedrez {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErroresAjedrez::Pieza => write!(f, "Pieza inválida"),
            ErroresAjedrez::Tablero => write!(f, "Tablero inválido"),
            ErroresAjedrez::Archivo => write!(f, "Archivo no encontrado"),
        }
    }
}
