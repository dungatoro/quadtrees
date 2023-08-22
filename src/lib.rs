#[derive(Clone, Debug)]
pub struct Quad {
    pub cell: Option<bool>,
    pub tree: Option<Box<[Quad;4]>>
}

pub fn quarter(matrix: Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let end = matrix.len();
    let half = end /2;

    let mut q1 = Vec::new();
    let mut q2 = Vec::new();
    let mut q3 = Vec::new();
    let mut q4 = Vec::new();

    for y in 0..half { let mut row = Vec::new(); for x in 0..half { row.push(matrix[y][x]); } q1.push(row); }
    
    for y in 0..half { let mut row = Vec::new(); for x in half..end { row.push(matrix[y][x]); } q2.push(row); }

    for y in half..end { let mut row = Vec::new(); for x in 0..half { row.push(matrix[y][x]); } q3.push(row); }

    for y in half..end { let mut row = Vec::new(); for x in half..end { row.push(matrix[y][x]); } q4.push(row); }

    vec![q1, q2, q3, q4]
}

pub fn build(matrix: Vec<Vec<bool>>) -> Quad {
    let mut quads: Quad = Quad::new();

    let qs = quarter(matrix);

    for i in 0..4 {
        quads.tree.as_mut().unwrap()[i] = {
            let pos = 
            qs[i].iter()
                 .position(|row| *row != qs[i][0]);

            match pos {
                None => Quad { cell: Some(qs[i][0][0]), tree: None },
                Some(_) => build(qs[i].clone())
            }
        }
    }

    quads

}

impl Quad {

    pub fn new() -> Self {
        let empty = Self { cell: None, tree: None };
        Self { cell: None, tree: Some( Box::new([empty.clone(), empty.clone(), empty.clone(), empty.clone()]) ) } 
    }

    pub fn from(matrix: Vec<Vec<bool>>) -> Option<Self> {
        // must be a square matrix of side length 2^n
        if matrix.len() != matrix[0].len() || matrix.len().count_ones() != 1 {
            return None
        }

        let quadtree = build(matrix);

        Some(quadtree)

    }
}

