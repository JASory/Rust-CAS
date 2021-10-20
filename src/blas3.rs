/*

 BLAS Level 3

*/

   use std::ops::*;
   use crate::traits::*;
   
   use crate::blas1::*;
   use crate::matrix::Matrix;

impl<T: SemiRing + PartialEq + Clone> Matrix<T>{

 pub fn unchecked_add(&mut self, other: &Self){
         add(&other.elements[..],&mut self.elements[..]);
 } 
 
 pub fn unchecked_vem(&mut self, other: &Self){
         vem(&other.elements[..],&mut self.elements[..]);
 }
       // matrix multiplication, zero checks on dimensions
 pub fn unchecked_gemm(&self, other: &Self)-> Self{
           let (row, col) = (self.row_len(), self.col_len());
      let mut z = Self::new(row, col, vec![]);

   for j in 0..col{
      for i in 0..col{
   let mut k = T::add_identity();
         for l in 0..row{
             k= k.clone() + self.elements[l+ (i*row)].clone() * other.elements[l*col+j].clone();
             }
      z.elements.push(k);
       }
     }
       z
   }
   
 }
 
 /*
 pub fn 
 
 pub fn gemm
 
 pub fn syrk
 
 pub fn 
 */
 impl<T: SemiRing + AddInverse + PartialEq + Clone> Matrix<T>{
 
 pub fn unchecked_sub(&mut self, other: &Self){
         sub(&other.elements[..], &mut self.elements[..]);
 }
 
}
