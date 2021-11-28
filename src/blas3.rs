/*

 BLAS Level 3  Matrix-Matrix Operations

*/

   use std::ops::*;
   use crate::traits::*;
   
   use crate::blas1::*;
   use crate::matrix::Matrix;
   
   

impl<T: SemiRing + PartialEq + Clone > Matrix<T>{

 pub fn unchecked_add(&mut self, other: &Self){
         add(&other.elements[..],&mut self.elements[..]);
 } 
        // hadamard multiplication check dimensions
 pub fn unchecked_vem(&mut self, other: &Self){
         vem(&other.elements[..],&mut self.elements[..]);
 }
       // matrix multiplication, zero checks on dimensions
 pub fn unchecked_gemm(&self, other: &Self)-> Self{
 
           let (row, col) = (self.row_num(), self.col_num());
           
      let mut z = Self::unchecked_new(row, col, vec![]);

   for j in 0..self.row_num(){
      for i in 0..self.row_num(){
   let mut k = T::add_identity();
         for l in 0..self.col_num(){
             k= k.clone() + self.elements[l+ (j*self.col_num())].clone() * other.elements[l*other.col_num()+i].clone();
             }
      z.elements.push(k);
       }
     }
       z
   } /*
    // Alternative implementation
  pub fn gemm(&self, other: &Self)->Self{
           let (row,col) = (self.row_num(),other.col_num());
           let mut z = Self::add_identity(row,col);
           
           for i in 0..col{
            for j in 0..row{
            z.elements[self.row_num()*j + i] = sdot(1,&self.elements[self.col_num()*j..self.col_num()*(1+j)],other.col_num(), &other.elements[i..])
            }
           } 
           z
  } */
   
  pub fn unchecked_kronecker_product(&self, other: &Self)->Self{
           let (row,col) = (self.row_num(), self.col_num());
           let mut interim = Self::unchecked_new(row*other.row_num(), col*other.col_num(), vec![]);
          for l in 0..self.row_num(){ 
           for k in 0..other.row_num(){
             for j in 0..self.col_num(){
              for i in 0..other.col_num(){
                interim.elements.push( self.elements[l*self.col_num() +j].clone() * other.elements[k*other.col_num() +i].clone() )
              }
             } 
            }
           } 
         interim
  } 
   
   // Check square
  pub fn unchecked_diagonal_det(&self)->T{
          let (mut product,m) = (T::mul_identity(), self.row_num());
          for i in 0..m{
             product = product * self.elements[(i*m+i) as usize].clone() ;
          }
          product
  } 
 }
  

 impl<T: SemiRing + AddInverse + PartialEq + Clone> Matrix<T>{
 
 pub fn unchecked_sub(&mut self, other: &Self){
         sub(&other.elements[..], &mut self.elements[..]);
 }
 
}

impl<T: Field + PartialEq + Clone> Matrix<T>{

 pub fn unchecked_diagonal_mul_inverse(&self)->Self{
         let mut k = self.clone();
         let m = self.row_num();
         
         for i in 0..m{
           k.elements[i*m + 1] =  k.elements[i*m + 1].mul_inverse()
         }
         k
 }


}
