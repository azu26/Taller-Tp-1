use super::puede_comer::PuedeComer;
use crate::Coordenada;

pub struct Caballo {
    coordenada: Coordenada,
}

impl Caballo {
    pub fn new(coordenada: Coordenada) -> Caballo {
        Caballo { coordenada }
    }
}

impl PuedeComer for Caballo {
    fn puede_comer(&self, coord2: Coordenada) -> bool {
        let coord1 = &self.coordenada;
        let resta_x = (coord1.get_x() - coord2.get_x()).abs();
        let resta_y = (coord1.get_y() - coord2.get_y()).abs();

        let posibilidades1 = resta_x == 2 && resta_y == 1;
        let posibilidades2 = resta_x == 1 && resta_y == 2;

        posibilidades1 || posibilidades2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caballo_puede_comer_pieza_en_diagonal() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(18);
        println!("{}, {}", coord1.get_x(), coord1.get_y());
        println!("{}, {}", coord2.get_x(), coord2.get_y());
        let caballo = Caballo::new(coord1);

        assert!(!caballo.puede_comer(coord2));
    }

    #[test]
    fn test_caballo_no_puede_comer_pieza_en_diagonal() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(20);
        let caballo = Caballo::new(coord1);

        assert!(caballo.puede_comer(coord2));
    }
}
