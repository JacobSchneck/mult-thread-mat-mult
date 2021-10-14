#![allow(
    dead_code,
    non_snake_case,
	 unused_variables,
)]

use std::thread;
use std::sync::Arc;

use crate::matrix::Matrix;
use crate::matrix_subtract::{self, matrix_subtract};
use crate::matrix_add::matrix_add;

pub fn mult_mat(A: &Matrix, B: &Matrix) -> Result<Matrix, String> {
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

fn split_matrix(matrix: &Matrix) -> Vec<Matrix> {
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

fn strassen(A: &Matrix, B: &Matrix) -> Matrix {
	if A.matrix.len() == 1 {
		return mult_mat(&A, &B).unwrap();
	}

	let split_A = split_matrix(A);
	let a = &split_A[0];
	let b = &split_A[1];
	let c = &split_A[2];
	let d = &split_A[3];

	let split_B = split_matrix(B);
	let e = &split_B[0];
	let f = &split_B[1];
	let g = &split_B[2];
	let h = &split_B[3];

	let p1 = &strassen(a, &matrix_subtract(f, h).unwrap());
	let p2 = &strassen(&matrix_add(a, b).unwrap(), h);
	let p3 = &strassen(&matrix_add(c, d).unwrap(), e);
	let p4 = &strassen(d, &matrix_subtract(g ,e).unwrap());
	let p5 = &strassen(&matrix_add(a, d).unwrap(), &matrix_add(e, h).unwrap());
	let p6 = &strassen(&matrix_subtract(b, d).unwrap(), &matrix_add(g, h).unwrap());
	let p7 = &strassen(&matrix_subtract(a, c).unwrap(), &matrix_add(e, f).unwrap());

	let c11 = matrix_subtract(&matrix_add(p5, p4).unwrap(), &matrix_add(p2, p6).unwrap()).unwrap();
	let c12 = matrix_add(p1, p2).unwrap();
	let c21 = matrix_add(p3, p4).unwrap();
	let c22 = matrix_subtract(&matrix_add(p1, p5).unwrap(), &matrix_subtract(p3, p7).unwrap()).unwrap();

	let result: Vec<Vec<i32>> = vec![];

	unimplemented!()
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

		let result = mult_mat(&A, &B).unwrap();
		assert_eq!(result.get_row(0), &[30, 36, 42]);
		assert_eq!(result.get_row(1), &[66, 81, 96]);
	}

	#[test]
	#[should_panic]
	fn mult_mat_panic_test() {
		let A: Matrix = Matrix::from(vec![vec![2, 3], vec![5, 6]]);
		let B: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

		let result = mult_mat(&A, &B);
		result.unwrap();
	}

	#[test]
	fn theory() {
		let test_vec = vec![1, 2, 3, 4];
		println!("{:?}", &test_vec[2..]);
	}
}
