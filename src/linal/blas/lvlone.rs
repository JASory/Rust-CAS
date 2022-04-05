/*

 BLAS level 1 Implemented for rings/semirings

*/

   use std::ops::*;
   use crate::traits::*;
 
   // Generalized scalar multiplication stores value in place
 pub fn scl<T: MulOp + Clone>(a: T, x: &mut[T]){
        for i in x.iter_mut(){
          *i= i.product( &a );
        }
       }
       
 pub fn sscl<T: MulOp + Clone>(a: T, x_stride: usize, x: &mut[T]){
          for i in x.iter_mut().step_by(x_stride){
             *i = i.product( &a );
          }
 } 
   // additive inverse of slice
 pub fn add_inv<T: AddInverse + Clone>(x: &mut [T]){
         for i in x.iter_mut(){
           *i = i.add_inverse()
         }
 }  

  
    // add 
 pub fn add<T: AddOp + Clone>(x: & [T], y: &mut [T]){ //
         for (i, j) in x.iter().zip(y.iter_mut()){
           *j= j.addition(&i)
         }
      }

     // strided in-place addition
 pub fn sadd<T: AddOp + Clone>(x_stride: usize, x: &[T], y_stride: usize, y: &mut [T]){
       for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
           *j= j.addition(&i)
      }

    }
    
 pub fn add_scale<T: AddOp + MulOp + Clone>(scale: T, x: &[T], y: &mut [T]){
         for (i,j) in x.iter().zip(y.iter_mut()){
           *j= scale.product(&i);
         }
      }   
    
     // subtraction by addition of inverse
 pub fn sub<T: AddOp + AddInverse + Clone>(x: & [T], y: &mut [T]){
         for (i, j) in x.iter().zip(y.iter_mut()){
           *j= j.addition(&i.add_inverse())
         }
      }     
      // strided subtraction
 pub fn ssub<T: AddOp + AddInverse + Clone>(x_stride: usize, x: & [T], y_stride: usize, y: &mut [T]){
       for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
           *j= j.addition(&i.add_inverse())
         }
      }     
      
 pub fn sub_scale<T: AddInverse + AddOp + MulOp + Clone>(scale: T, x: &[T], y: &mut [T]){
         for (i,j) in x.iter().zip(y.iter_mut()){
           *j= j.addition( &scale.product( &i.add_inverse() ) );
         }
      }             

     // in-place Hadamard
pub fn vem<T: MulOp + Clone>(x: & [T], y :&mut [T] ){
     for (i,j) in x.iter().zip(y.iter_mut()){
        *j= j.product(&i)
      }
    }

    // In-place strided Hadamard , stores in mutable slice y
 pub fn svem<T: MulOp + Clone>(x_stride: usize ,x: & [T], y_stride: usize, y :&mut [T] ){
     for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
        *j= j.product(i)
      }
    }
     // In-place Inverse Hadamard
 pub fn ivem<T: Field + Clone>(x: &[T], y: &mut [T]){
     for (i,j) in x.iter().zip(y.iter_mut()){
      *j = j.product(&i.mul_inverse())
     }   
 }   
 
 pub fn sivem<T: Field + Clone>(x_stride: usize ,x: &[T], y_stride: usize, y :&mut [T] ){
     for (i,j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){
        *j= j.product(&i.mul_inverse())
      }
    }

 pub fn axpy<T: AddOp + MulOp + Copy >(a: T, x: &[T] , y: &mut [T] ){
         for (i, j) in x.iter().zip(y.iter_mut()){
            *j= j.addition(&a.product(&i ))
          }
        }


 // Generalized strided axpy operation, stores value in mutable y slice
 pub fn saxpy<T: AddOp + MulOp + Copy >(a: T, x_stride: usize, x: &[T], y_stride : usize , y: &mut [T] ){ 

       for (i, j) in x.iter().step_by(x_stride).zip(y.iter_mut().step_by(y_stride)){ 
         *j= j.addition( &a.product(&i) )
       }
    }  
         //
 pub fn dot<T: SemiRing + Clone >(x: &[T], y: & [T])->T{
     let mut k = T::default().add_identity();
     for (i, j) in x.iter().zip(y.iter()){
         k = k.addition( &i.product(&j) )
     }
     k
   }

        // strided dot product
 pub fn sdot<T: SemiRing + Clone> (x_stride: usize, x: &[T], y_stride: usize,  y: & [T])->T{
          let mut k = T::default().add_identity();
      for (i,j) in x.iter().step_by(x_stride).zip(y.iter().step_by(y_stride)){
              k = k.addition(&i.product(&j))
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
  // gcd vector
 pub fn vector_gcd<T: Euclidean + Clone>(x: &[T])->T{
          let mut tmp = x[0].gcd(&x[1]);
        for i in 2..x.len(){
          tmp = x[i].gcd(&tmp);
        }
        tmp
 }
 
 pub fn gcd_reduce<T: Euclidean + Clone>(x: &mut[T])->T{
         let gcd = vector_gcd(x);
         for i in x.iter_mut(){
           *i = i.clone().euclidean(&gcd).0;
         }
         gcd
 }
