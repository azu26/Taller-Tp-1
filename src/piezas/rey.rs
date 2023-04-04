use super::puede_comer::PuedeComer;
use crate::Coordenada;

pub struct Rey {
    coordenada: Coordenada,
}

impl Rey {
    pub fn new(coordenada: Coordenada) -> Rey {
        Rey { coordenada }
    }
}

impl PuedeComer for Rey {
    fn puede_comer(&self, coord2: Coordenada) -> bool {
        let coord1 = &self.coordenada;
        let resta_x = (coord1.get_x() - coord2.get_x()).abs();
        let resta_y = (coord1.get_y() - coord2.get_y()).abs();

        resta_x <= 1 && resta_y <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rey_puede_comer_pieza_ady_en_diagonal() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(18);
        println!("{}, {}", coord1.get_x(), coord1.get_y());
        println!("{}, {}", coord2.get_x(), coord2.get_y());
        let rey = Rey::new(coord1);

        assert!(rey.puede_comer(coord2));
    }

    #[test]
    fn test_rey_puede_comer_pieza_ady_ortogonal() {
        let coord1 = Coordenada::new(34);
        let coord2 = Coordenada::new(36);
        let rey = Rey::new(coord1);

        assert!(rey.puede_comer(coord2));
    }

    #[test]
    fn test_rey_no_puede_comer_pieza_alejada() {
        let coord1 = Coordenada::new(30);
        let coord2 = Coordenada::new(32);
        let rey = Rey::new(coord1);

        assert!(!rey.puede_comer(coord2));
    }
}
