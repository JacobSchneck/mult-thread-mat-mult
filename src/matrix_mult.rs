#![allow(
    dead_code,
    non_snake_case,
	 unused_variables,
)]

use std::thread;
use std::sync::Arc;

use crate::matrix::Matrix;

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

fn split_matrix(matrix: Matrix) -> Vec<Matrix> {
	let new_m = matrix.m / 2;
	let new_n = matrix.n / 2;

	// new matricies takes form
	// 1) 0..new_m, 0..new_n
	// 2) 0..new_m, new_n..
	// 3) new_m.., 0..new_n
	// 4) new_m.., new_n..

	let mut result: Vec<Matrix> = vec![];
	result.push(Matrix::from(matrix.matrix[0..new_m][0..new_n].to_vec()));
	result.push(Matrix::from(matrix.matrix[0..new_m][new_n..].to_vec()));
	result.push(Matrix::from(matrix.matrix[new_m..][0..new_n].to_vec()));
	result.push(Matrix::from(matrix.matrix[new_m..][new_n..].to_vec()));

	result
}

fn strassen(A: Matrix, B: Matrix) -> Matrix {
	if A.len() == 1 {
		return A[0] 
	}
}

pub fn mult_mat_div_and_conq(A: Matrix, B: Matrix) -> Result<Matrix, String> {

	unimplemented!();
}

pub fn concurrent_mult_mat(A: Matrix, B: Matrix, pool: u32) -> Result<Matrix, String> {
    if A.n != B.m {
        let message = format!("Matrix {}x{} cannot be multiplied matrix {}x{}", A.m, A.n, B.m, B.n);
        return Err(message);
    }

    for _ in 0..pool {
        thread::spawn(|| {

        });
    }

    unimplemented!();
}


#[cfg(test)]
pub mod test_matrix_mult {
	use super::*;

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

	#[test]
	fn theory() {
		let test_vec = vec![1, 2, 3, 4];
		println!("{:?}", &test_vec[2..]);
	}
}
