#![allow(
    dead_code,
    non_snake_case
)]

pub type Mat = Vec<Vec<i32>>;

pub struct Matrix {
    transpose: Mat,
    pub m: usize,
    pub n: usize,
    matrix: Mat,
}

fn transpose(nested_vec: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![0; nested_vec.len()]; nested_vec[0].len()];
    for (r, row) in nested_vec.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            result[c][r] = *val;
        }
    }
    result
}

impl Matrix {
    pub fn from(nested_vec: Mat) -> Self {
        Matrix {
            transpose: transpose(&nested_vec),
            m: nested_vec.len(),
            n: nested_vec[0].len(),
            matrix: nested_vec,
        }
    }

    pub fn get_row(&self, index: usize) -> &Vec<i32> {
        &self.matrix[index]
    }

    pub fn get_col(&self, index: usize) -> &Vec<i32> {
        &self.transpose[index]
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
        let mut str = String::new();
        for row in &self.matrix {
            let mut row_str = String::new();
            for val in row {
                row_str = row_str.to_owned() + &format!("{} ", val);
            }
            str = str + &format!("{}\n", row_str);
        }
        write!(f, "{}", str)
    }
}