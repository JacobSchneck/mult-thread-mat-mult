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

pub fn mult_mat_div_and_conq(A: Matrix, B: Matrix) -> Result<Matrix, String> {
	unimplemented!();
}

pub fn concurrent_mult_mat(A: Matrix, B: Matrix, pool: u32) -> Result<Matrix, String> {
    if A.n != B.m {
        let message = format!("Matrix {}x{} cannot be multiplied matrix {}x{}", A.m, A.n, B.m, B.n);
        return Err(message);
    }

    for _ in pool {
        thread::spawn(|| {

        })
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
