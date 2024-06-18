pub use lalgebra_scalar::Scalar;
#[derive(Debug, Clone,PartialEq,Eq)]                                             
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero(); 1]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut data = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            data[i][i] = T::one();
        }
        Matrix(data)
    }
}

