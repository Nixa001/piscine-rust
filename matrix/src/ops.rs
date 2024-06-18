use std::ops::Mul;
use lalgebra_scalar::*;

#[derive(Debug, PartialEq,Eq,Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        }
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T>{
        // if n < self.number_of_rows() {
            &mself.0[n]
        // } 
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        // if n < self.number_of_cols() {
            let mut col = Vec::with_capacity(self.number_of_rows());
            for i in 0..self.number_of_rows() {
                col.push(self.0[i][n].clone());
            }
            col
        }
    // }
}

impl<T: Scalar<Item = T>> Mul for Matrix<T> where T: Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None; // Invalid matrix dimensions for multiplication
        }

        let mut result = vec![];
        for i in 0..self.number_of_rows() {
            let mut row = vec![];
            for j in 0..other.number_of_cols() {
                let mut sum = T::default();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k].clone() * other.0[k][j].clone();
                }
                row.push(sum);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}
