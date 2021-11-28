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
       
 pub fn sscl<T: Mul<Output = T> + Clone>(a: T, x_stride: usize, x: &mut[T]){
          for i in x.iter_mut().step_by(x_stride){
             *i = i.clone() * a.clone();
          }
 } 
   // additive inverse of slice
 pub fn add_inv<T: AddInverse + Clone>(x: &mut [T]){
         for i in x.iter_mut(){
           *i = i.add_inverse()
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
    
 pub fn add_scale<T: Add<Output = T> + Mul<Output =T> + Clone>(scale: T, x: &[T], y: &mut [T]){
         for (i,j) in x.iter().zip(y.iter_mut()){
           *j= scale.clone() * i.clone();
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
      
 pub fn sub_scale<T: AddInverse + Add<Output = T> + Mul<Output =T> + Clone>(scale: T, x: &[T], y: &mut [T]){
         for (i,j) in x.iter().zip(y.iter_mut()){
           *j= j.clone() + (scale.clone() * i.clone().add_inverse());
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
     // In-place Inverse Hadamard
 pub fn ivem<T: Field + Clone>(x: &[T], y: &mut [T]){
     for (i,j) in x.iter().zip(y.iter_mut()){
      *j = j.clone() * i.clone().mul_inverse()
     }   
 }   
 
 pub fn sivem<T: Field + Clone>(x_stride: usize ,x: &[T], y_stride: usize, y :&mut [T] ){
     for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
        *j= j.clone() *i.clone().mul_inverse()
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
 pub fn dot<T: SemiRing + Clone >(x: &[T], y: & [T])->T{
     let mut k = T::add_identity();
     for (i, j) in x.iter().zip(y.iter()){
         k = k.clone() + i.clone()*j.clone()
     }
     k
   }

        // strided dot product
 pub fn sdot<T: SemiRing + Clone> (x_stride: usize, x: &[T], y_stride: usize,  y: & [T])->T{
          let mut k = T::add_identity();
      for (i,j) in x.iter().step_by(x_stride).zip(y.iter().step_by(y_stride)){
              k = k.clone() + i.clone()*j.clone()
    }
    k
    }
    
    
 pub fn norm(x: &[f64])->f64{
        let mut sum = 0f64;
      for i in x.iter(){
          sum+=i*i;
      }
      sum.sqrt()
 }
 
 pub fn normalize(x: &mut[f64]){
         let norm = norm(x);
         for i in x.iter_mut(){
            *i /=norm
         }
 }   
