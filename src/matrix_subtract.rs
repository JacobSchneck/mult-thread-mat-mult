#![allow(non_snake_case)]

use crate::matrix::Matrix;

pub fn matrix_subtract(A: &Matrix, B: &Matrix) -> Result<Matrix, &'static str> {
	if A.m != B.m && A.n != B.n {
		return Err("Matrix shapes do not match");
	}

	let mut result = vec![vec![0; A.n]; A.m];

	for i in 0..A.m {
		for j in 0..B.m {
			result[i][j] = A.matrix[i][j] - B.matrix[i][j];
		}
	} 
	Ok(Matrix::from(result))
}