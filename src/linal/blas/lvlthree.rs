
   use crate::traits::*;

pub fn unchecked_gemm<T: SemiRing >(m: usize, n: usize, x: &[T], y: &[T], z: &mut [T]){
       for i in 0..m {
        for j in 0..m {
         for k in 0..n {
            z[m*i + j] = z[m*i + j].addition(&x[n*i +k].product( &y[m*k + j]) ) ;
         }
        }
       }     
 }
 

   
