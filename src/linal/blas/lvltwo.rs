

   use crate::traits::*;
   
   
   // x by vector y
 pub fn unchecked_gemv<T: SemiRing >(m: usize, n: usize, x: &[T], y: &[T], z: &mut [T]){
 
        for j in 0..m {
         for k in 0..n {
            z[j] = z[j].addition(&x[n*j +k].product( &y[k]) ) ;
         }
        }    
 
 }
