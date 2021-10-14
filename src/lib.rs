#![allow(
    dead_code,
    non_snake_case
)]

pub type Mat = Vec<Vec<i32>>;
pub struct Matrix {
    transpose: Mat,
    m: usize,
    n: usize,
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

pub fn mult_mat(A: Matrix, B: Matrix) -> Result<Matrix, String> {
    if A.n != B.m {
        let message = format!("Matrix {}x{} cannot be multiplied matrix {}x{}", A.m, A.n, B.m, B.n);
        return Err(message);
    }

    let mut result = vec![vec![0; B.n]; A.m];
    for i in 0..A.m {
        let row = A.get_row(i);
        for j in 0..B.n {
            let col = B.get_col(j);
            let mut val = 0;
            for k in 0..col.len() {
                val += col[k]*row[k];
            }
            result[i][j] = val;
        }
    }

    Ok(Matrix::from(result))
}

pub fn concurrent_mult_mat(A: Matrix, B: Matrix, pool: u32) -> Result<Matrix, String> {
    if A.n != B.m {
        let message = format!("Matrix {}x{} cannot be multiplied matrix {}x{}", A.m, A.n, B.m, B.n);
        return Err(message);
    }


    unimplemented!();
}

#[cfg(test)]
mod test_matrix {
    use super::*;

    #[test] 
    fn basic_test() {
        let matrix: Matrix = Matrix::from(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(matrix.get_row(0), &[1, 0]);
        assert_eq!(matrix.get_row(1), &[0, 1]);

        assert_eq!(matrix.get_col(0), &[1, 0]);
        assert_eq!(matrix.get_col(1), &[0, 1]);
        
        let matrix: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(matrix.get_row(0), &[1, 2, 3]);
        assert_eq!(matrix.get_row(1), &[4, 5, 6]);

        assert_eq!(matrix.get_col(0), &[1, 4]);
        assert_eq!(matrix.get_col(1), &[2, 5]);
        assert_eq!(matrix.get_col(2), &[3, 6]);
    }

    #[test]
    fn mult_mat_test() {
        let A: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let B: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        let result = mult_mat(A, B).unwrap();
        assert_eq!(result.get_row(0), &[30, 36, 42]);
        assert_eq!(result.get_row(1), &[66, 81, 96]);
    }

    #[test]
    #[should_panic]
    fn mult_mat_panic_test() {
        let A: Matrix = Matrix::from(vec![vec![2, 3], vec![5, 6]]);
        let B: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        let result = mult_mat(A, B);
        result.unwrap();
    }
}

