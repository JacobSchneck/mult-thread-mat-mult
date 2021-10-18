
#![allow(
    dead_code,
    non_snake_case,
	 unused_variables,
	 unused_imports,
)]

extern crate num_cpus;

use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

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

pub fn partition_rows(m: usize, pool: usize) -> Vec<Vec<usize>> {
	// partition data
	let set  = m / pool;
	let leftover = m % pool;

	
	let mut rows: Vec<Vec<usize>> = Vec::new();
	let mut rows_partitioned = 0;
	while rows_partitioned < m {
		if rows_partitioned + set > m {
			for i in rows_partitioned..m {
				let last_index = rows.len() - 1;
				rows[last_index].push(i);
				rows_partitioned += 1;
			}
		} else {
			rows.push((rows_partitioned..(rows_partitioned + set)).collect::<Vec<usize>>());
			rows_partitioned += set;
		}
	}
	rows
}

pub fn concurrent_mult_mat(A: Matrix, B: Matrix) -> Result<Matrix, String> {
	if A.n != B.m {
	let message = format!("Matrix {}x{} cannot be multiplied matrix {}x{}", A.m, A.n, B.m, B.n);
	return Err(message);
	}

	// why bother with multithreading in this case;
	let pool = num_cpus::get();
	if A.m <= pool {
	return mult_mat(&A, &B);
	}

	let partitions = partition_rows(A.m, pool);

	let mut handles: Vec<JoinHandle<Result<Vec<i32>, &str>>> = vec![];

	for partition in partitions {
		// HACK: Having to clone the matrix for every instance to prevent angry borrow checker
		let A = A.clone(); 
		let B = B.clone();
		let partition = partition.clone();
		handles.push(thread::spawn(move || {
			let mut result = Vec::<i32>::new();

			for row_index in partition {
				let row_vals = A.get_row(row_index);
				for col_index in 0..B.n {
					let col_vals = B.get_col(col_index);
					let mut val = 0;
					for i in 0..row_vals.len() {
						val += row_vals[i] * col_vals[i];
					}
					result.push(val);
				}
			}
			Ok(result)
		}));
	}


	let mut full_result: Vec<Vec<i32>> = Vec::new();
	for handle in handles {
		let result = handle.join().unwrap().unwrap();
		full_result.push(result);
	}

	Ok(Matrix::from(full_result))
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
	fn test_partition_rows() {
		// println!("{:?}", (0..10).collect::<Vec<usize>>());
		assert_eq!(partition_rows(12, 4), &[
			[0, 1, 2], 
			[3, 4, 5], 
			[6, 7, 8], 
			[9, 10, 11]
		]);

		assert_eq!(partition_rows(13, 4), vec![
			vec![0, 1, 2], 
			vec![3, 4, 5], 
			vec![6, 7, 8], 
			vec![9, 10, 11, 12]
		]);

		assert_eq!(partition_rows(13, 3), vec![
			vec![0, 1, 2, 3], 
			vec![4, 5, 6, 7], 
			vec![8, 9, 10, 11, 12]
		]);
	}

	#[test]
	fn test_concurrent_mat_mult() {
		// Basic Test
		let A: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
		let B: Matrix = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

		let result = concurrent_mult_mat(A, B).unwrap();
		assert_eq!(result.get_row(0), &[30, 36, 42]);
		assert_eq!(result.get_row(1), &[66, 81, 96]);
		let A: Matrix = Matrix::from(vec![
			vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
			vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0], 
			vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0], 
			vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0], 
			vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], 
			vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], 
			vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0], 
			vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0], 
			vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0], 
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
		]);
		let B: Matrix = Matrix::from(vec![
			vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
			vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0], 
			vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0], 
			vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0], 
			vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], 
			vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], 
			vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0], 
			vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0], 
			vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0], 
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
		]);
		let result = concurrent_mult_mat(A, B);
		println!("{}", result.unwrap());

		// Confirmed using numpy
		let A: Matrix = Matrix::from(vec![
			vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
			vec![1, 1, 0, 0, 0, 0, 0, 0, 1, 1], 
			vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1], 
			vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1], 
			vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 1], 
			vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 1], 
			vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1], 
			vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1], 
			vec![1, 1, 0, 0, 0, 0, 0, 0, 1, 1], 
			vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
		]);
		let B: Matrix = Matrix::from(vec![
			vec![1, 0, 1, 0,], 
			vec![0, 1, 0, 0,], 
			vec![1, 0, 1, 0,], 
			vec![1, 0, 0, 1,], 
			vec![0, 1, 0, 1,], 
			vec![0, 0, 1, 0,], 
			vec![0, 1, 0, 1,], 
			vec![1, 0, 0, 1,], 
			vec![1, 0, 1, 0,], 
			vec![0, 1, 0, 0,], 
		]);
		let result = concurrent_mult_mat(A, B);
		println!("{}", result.unwrap());
	}

	#[test]
	fn theory() {
		let test_vec = vec![1, 2, 3, 4];
		println!("{:?}", &test_vec[2..]);
	}
}
