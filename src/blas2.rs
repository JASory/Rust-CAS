/*

  BLAS Level 2 matrix-vector operations

*/

   use std::ops::*;
   use crate::traits::*;
   use crate::blas1::*;
   use crate::matrix::Matrix;

// Matrix vector operations
impl<T: SemiRing + PartialEq + Clone > Matrix<T>{

         // matrix vector multiply
  pub fn unchecked_gemv(&self, x: &Vec<T>)->Self{
           let (m,n) = (self.row_num(),self.col_num());
           let mut interim = Matrix::<T>::unchecked_new(1,m,vec![]);

          for i in 0..m{
             interim.elements.push(dot(&self.elements[(n*i)..(n*(i+1))],&x[..]))
          }
          interim
  } 
  
  pub fn unchecked_gaxpy(&self,x: &Vec<T>, y: &Vec<T>)->Self{
           let (m,n) = (self.row_num(),self.col_num());
           let mut interim = Matrix::<T>::unchecked_new(1,m,vec![]);

          for i in 0..m{
             interim.elements.push(dot(&self.elements[(n*i)..(n*(i+1))],&x[..]) + y[i].clone())
          }
          interim
  }/*
  
  pub fn outer_product
         // rank 1 operation
  pub fn ger
         // rank 2 operation (symmetric)?
  pub fn syr */
}


