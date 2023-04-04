use super::puede_comer::PuedeComer;
use crate::Coordenada;

pub enum Color {
    Blanco,
    Negro,
}

pub struct Peon {
    coordenada: Coordenada,
    color: Color,
}

impl Peon {
    pub fn new(color: Color, coordenada: Coordenada) -> Peon {
        Peon { color, coordenada }
    }
}

impl PuedeComer for Peon {
    fn puede_comer(&self, coord2: Coordenada) -> bool {
        let coord1 = &self.coordenada;
        let c1x = coord1.get_x();
        let c1y = coord1.get_y();
        let c2x = coord2.get_x();
        let c2y = coord2.get_y();

        let blanco_puede_comer = c2y == (c1y - 1) && (c2x == (c1x - 1) || (c2x == (c1x + 1)));
        let negro_puede_comer = c2y == (c1y + 1) && (c2x == (c1x - 1) || (c2x == (c1x + 1)));

        match self.color {
            Color::Blanco => blanco_puede_comer,
            Color::Negro => negro_puede_comer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peon_negro_puede_comer_pieza_en_diagonal() {
        let coord1 = Coordenada::new(32);
        let coord2 = Coordenada::new(50);
        println!("{}, {}", coord1.get_x(), coord1.get_y());
        println!("{}, {}", coord2.get_x(), coord2.get_y());
        let peon = Peon::new(Color::Negro, coord1);

        assert!(peon.puede_comer(coord2));
    }

    #[test]
    fn test_peon_blanco_puede_comer_pieza_en_diagonal() {
        let coord1 = Coordenada::new(64);
        let coord2 = Coordenada::new(50);
        let peon = Peon::new(Color::Blanco, coord1);

        assert!(peon.puede_comer(coord2));
    }

    #[test]
    fn test_peon_no_puede_comer_pieza_contigua() {
        let coord1 = Coordenada::new(64);
        let coord2 = Coordenada::new(48);
        let peon = Peon::new(Color::Blanco, coord1);

        assert!(!peon.puede_comer(coord2));
    }
}
