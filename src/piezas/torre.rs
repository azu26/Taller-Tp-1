use super::puede_comer::PuedeComer;
use crate::Coordenada;

pub struct Torre {
    coordenada: Coordenada,
}

impl Torre {
    // Another associated function, taking two arguments:
    pub fn new(coordenada: Coordenada) -> Torre {
        Torre { coordenada }
    }
}

impl PuedeComer for Torre {
    fn puede_comer(&self, coord2: Coordenada) -> bool {
        let coord1 = &self.coordenada;
        coord1.get_x() == coord2.get_x() || coord1.get_y() == coord2.get_y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torre_puede_comer_pieza_vertical() {
        let coord1 = Coordenada::new(6);
        let coord2 = Coordenada::new(38);
        println!("{}, {}", coord1.get_x(), coord1.get_y());
        println!("{}, {}", coord2.get_x(), coord2.get_y());
        let torre = Torre::new(coord1);

        assert!(torre.puede_comer(coord2));
    }

    #[test]
    fn test_torre_puede_comer_pieza_horizontal() {
        let coord1 = Coordenada::new(48);
        let coord2 = Coordenada::new(58);
        let torre = Torre::new(coord1);

        assert!(torre.puede_comer(coord2));
    }

    #[test]
    fn test_torre_no_puede_comer_pieza_diagonal() {
        let coord1 = Coordenada::new(34);
        let coord2 = Coordenada::new(52);
        let torre = Torre::new(coord1);

        assert!(!torre.puede_comer(coord2));
    }
}
