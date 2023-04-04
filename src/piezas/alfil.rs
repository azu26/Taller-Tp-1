use super::puede_comer::PuedeComer;
use crate::Coordenada;

pub struct Alfil {
    coordenada: Coordenada,
}

impl Alfil {
    pub fn new(coordenada: Coordenada) -> Alfil {
        Alfil { coordenada }
    }
}

impl PuedeComer for Alfil {
    fn puede_comer(&self, coord2: Coordenada) -> bool {
        let coord1 = &self.coordenada;
        let resta_x = (coord1.get_x() - coord2.get_x()).abs();
        let resta_y = (coord1.get_y() - coord2.get_y()).abs();

        resta_x == resta_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alfil_puede_comer_pieza_en_diagonal() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(18);
        let alfil = Alfil::new(coord1);

        assert!(alfil.puede_comer(coord2));
    }

    #[test]
    fn test_alfil_no_puede_comer_pieza_en_diagonal() {
        let coord1 = Coordenada::new(0);
        let coord2 = Coordenada::new(22);
        let alfil = Alfil::new(coord1);

        assert!(!alfil.puede_comer(coord2));
    }
}
