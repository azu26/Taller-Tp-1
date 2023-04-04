use super::puede_comer::PuedeComer;
use crate::Coordenada;

pub struct Dama {
    coordenada: Coordenada,
}

impl Dama {
    pub fn new(coordenada: Coordenada) -> Dama {
        Dama { coordenada }
    }
}

impl PuedeComer for Dama {
    fn puede_comer(&self, coord2: Coordenada) -> bool {
        let coord1 = &self.coordenada;
        let resta_x = (coord1.get_x() - coord2.get_x()).abs();
        let resta_y = (coord1.get_y() - coord2.get_y()).abs();
        let puede_comer_ortogonal =
            coord1.get_x() == coord2.get_x() || coord1.get_y() == coord2.get_y();

        resta_x == resta_y || puede_comer_ortogonal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dama_puede_comer_pieza_con_mov_diagonal() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(36);
        println!("{}, {}", coord1.get_x(), coord1.get_y());
        println!("{}, {}", coord2.get_x(), coord2.get_y());
        let dama = Dama::new(coord1);

        assert!(dama.puede_comer(coord2));
    }

    #[test]
    fn test_dama_puede_comer_pieza_con_mov_lateral() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(48);
        let dama = Dama::new(coord1);

        assert!(dama.puede_comer(coord2));
    }

    #[test]
    fn test_dama_no_puede_comer_pieza() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(20);
        let dama = Dama::new(coord1);

        assert!(!dama.puede_comer(coord2));
    }
}
