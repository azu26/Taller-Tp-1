use crate::coordenada::Coordenada;

pub trait PuedeComer {
    fn puede_comer(&self, coord2: Coordenada) -> bool;
}
