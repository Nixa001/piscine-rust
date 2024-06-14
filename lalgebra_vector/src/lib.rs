pub use lalgebra_scalar::Scalar;
use std::ops::Add;
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Vector<T: Scalar>(pub Vec<T>);



impl<T : Scalar + std::ops::Add> Add for Vector<T> where Vec<T>: FromIterator<<T as Add>::Output>{
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None
        }

        let result = self.0.into_iter().zip(other.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Some(Vector(result))
    }
   
}

impl <T:Scalar<Item =T> + std::ops::AddAssign+std::ops::Mul<Output = T> + std::ops::Add<Output = T>> Vector<T>{
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let mut result = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result = result + (*a) * (*b);
        }
        Some(result)
    }
    
}