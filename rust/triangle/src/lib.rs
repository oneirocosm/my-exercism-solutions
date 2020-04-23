use std::collections::HashSet;

pub struct Triangle {
    sides: Vec<u64>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if !Self::side_lengths_valid(&sides) || !Self::triangle_inequality_valid(&sides) {
            return None;
        }

        Some(Triangle {
            sides: sides.to_vec(),
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides
            .clone()
            .into_iter()
            .collect::<HashSet<u64>>()
            .len()
            == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides
            .clone()
            .into_iter()
            .collect::<HashSet<u64>>()
            .len()
            == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides
            .clone()
            .into_iter()
            .collect::<HashSet<u64>>()
            .len()
            < 3
    }

    fn side_lengths_valid(sides: &[u64; 3]) -> bool {
        sides.iter().all(|&side| side != 0)
    }

    fn triangle_inequality_valid(sides: &[u64; 3]) -> bool {
        let max_side = sides.iter().max().unwrap();
        let perimeter = sides.iter().sum();
        2 * max_side < perimeter
    }
}
