/*

 BLAS level 1 Implemented for rings/semirings

*/

   use std::ops::*;
   use crate::traits::*;
 
   // Generalized scalar multiplication stores value in place
  pub fn scl<T: Mul<Output =T> + Clone>(a: T, x: &mut[T]){
        for i in x.iter_mut(){
          *i= i.clone() *a.clone();
        }
       }
      

  
    // add 
 pub fn add<T: Add<Output = T> + Clone>(x: & [T], y: &mut [T]){ //
         for (i, j) in x.iter().zip(y.iter_mut()){
           *j= j.clone() + i.clone()
         }
      }

     // strided in-place addition
 pub fn sadd<T: Add<Output = T> + Clone>(x_stride: usize, x: &[T], y_stride: usize, y: &mut [T]){
       for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
           *j= j.clone() + i.clone()
      }

    }
    
     // subtraction by addition of inverse
 pub fn sub<T: Add<Output =T> + AddInverse + Clone>(x: & [T], y: &mut [T]){
         for (i, j) in x.iter().zip(y.iter_mut()){
           *j= j.clone() + i.add_inverse()
         }
      }     
      // strided subtraction
 pub fn ssub<T: Add<Output =T> + AddInverse + Clone>(x_stride: usize, x: & [T], y_stride: usize, y: &mut [T]){
       for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
           *j= j.clone() + i.add_inverse()
         }
      }          

     // in-place Hadamard
pub fn vem<T: Mul<Output = T> + Clone>(x: & [T], y :&mut [T] ){
     for (i,j) in x.iter().zip(y.iter_mut()){
        *j= j.clone() *i.clone()
      }
    }

    // In-place strided Hadamard , stores in mutable slice y
 pub fn svem<T: Mul<Output = T> + Clone>(x_stride: usize ,x: & [T], y_stride: usize, y :&mut [T] ){
     for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
        *j= j.clone() *i.clone()
      }
    }

 pub fn axpy<T: Add<Output =T> + Mul<Output=T> + Copy >(a: T, x: &[T] , y: &mut [T] ){
         for (i, j) in x.iter().zip(y.iter_mut()){
            *j= j.clone() + a.clone() * i.clone()
          }
        }


 // Generalized strided axpy operation, stores value in mutable y slice
 pub fn saxpy<T: Add<Output =T> + Mul<Output=T> + Copy >(a: T, x_stride: usize, x: &[T], y_stride : usize , y: &mut [T] ){ 

       for (i, j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){ 
         *j= j.clone() + a.clone() * i.clone()
       }
    }  
         //
 pub fn dot<T: Ring + Clone >(x: &[T], y: & [T])->T{
     let mut k = T::add_identity();
     for (i, j) in x.iter().zip(y.iter()){
         k = k.clone() + i.clone()*j.clone()
     }
     k
   }

        // strided dot product
 pub fn sdot<T: Ring + Clone> (x_stride: usize, x: &[T], y_stride: usize,  y: & [T])->T{
          let mut k = T::add_identity();
      for (i,j) in x.iter().step_by(x_stride).zip(y.iter().step_by(y_stride)){
              k = k.clone() + i.clone()*j.clone()
    }
    k
    }
