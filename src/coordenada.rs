pub struct Coordenada {
    x: i32,
    y: i32,
}

impl Coordenada {
    // Another associated function, taking two arguments:
    pub fn new(indice: usize) -> Coordenada {
        let x = (((indice % 16 + 1) as f32) / 2.0).ceil() as i32; //prueba%16 +1 = pos x () prueba / 16 redondeado para arriba esy
        let y = match indice % 16 {
            0 => ((indice as f32) / 16.0).ceil() as i32 + 1,
            _ => ((indice as f32) / 16.0).ceil() as i32,
        };

        Coordenada { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordenada_corresponde_posicion1() {
        let coord1 = Coordenada::new(0);
        println!("{}, {}", coord1.get_x(), coord1.get_y());

        assert!(coord1.get_x() == 1 && coord1.get_y() == 1);
    }

    #[test]
    fn test_coordenada_corresponde_posicion2() {
        let coord1 = Coordenada::new(18);
        println!("{}, {}", coord1.get_x(), coord1.get_y());

        assert!(coord1.get_x() == 2 && coord1.get_y() == 2);
    }

    #[test]
    fn test_coordenada_corresponde_posicion3() {
        let coord1 = Coordenada::new(52);
        println!("{}, {}", coord1.get_x(), coord1.get_y());

        assert!(coord1.get_x() == 3 && coord1.get_y() == 4);
    }

    #[test]
    fn test_coordenada_corresponde_posicion4() {
        let coord1 = Coordenada::new(32);
        println!("{}, {}", coord1.get_x(), coord1.get_y());

        assert!(coord1.get_x() == 1 && coord1.get_y() == 3);
    }
}
