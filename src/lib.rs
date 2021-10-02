pub type Mat = Vec<Vec<i32>>;

pub struct Matrix {
    matrix: Mat,
    row: i32,
    col: i32,
    col_ind: Vec<i32>,
}

impl Matrix {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn from(nested_vec: Mat) -> Self {
        unimplemented!();
    }

    fn calculate_column_indicies() {
        unimplemented!();
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
}

