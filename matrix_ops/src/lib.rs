use lalgebra_scalar::*;
use std::ops::{ Add, Sub };

#[derive(Debug, PartialEq,Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None; // Matrices have different dimensions
        }

        let mut result = vec![];
        for (row1, row2) in self.0.iter().zip(other.0.iter()) {
            let mut new_row = vec![];
            for (val1, val2) in row1.iter().zip(row2.iter()) {
                new_row.push(*val1 + *val2);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}

impl <T: Scalar<Item = T> + Add<Output = T>> Sub for Matrix <T>{
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None; // Matrices have different dimensions
        }

        let mut result = vec![];
        for (row1, row2) in self.0.iter().zip(other.0.iter()) {
            let mut new_row = vec![];
            for (val1, val2) in row1.iter().zip(row2.iter()) {
                new_row.push(*val1 - *val2);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }

}