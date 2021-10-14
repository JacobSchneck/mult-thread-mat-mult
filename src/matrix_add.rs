#![allow(non_snake_case)]
use crate::matrix::Matrix;

pub fn matrix_add(A: &Matrix, B: &Matrix) -> Result<Matrix, String> {
	if A.m != B.m && A.n != B.n {
		return Err("Matrix shapes do not match".to_string());
	}

	let mut result = vec![vec![0; A.n]; A.m];

	for i in 0..A.m {
		for j in 0..B.m {
			result[i][j] = A.matrix[i][j] + B.matrix[i][j];
		}
	} 
	Ok(Matrix::from(result))
}