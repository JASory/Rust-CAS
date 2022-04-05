/*
   Matrix ops shared by both square and non-square matrices for code simplicity
*/

  use crate::traits::*;

pub fn matrix_format<T: SemiRing>(m: usize, n: usize, mat: &[T]) -> String{
     let mut k = Vec::new();
        for i in 0..m{
         k.push(mat[n*i..n*(i+1)].iter().map(|x|
          x.format() ).collect::<Vec<String>>().join(","))
            }
           "\n".to_string() + &k.join("\n")
        }
        
        
pub fn permute_diagonal<T: SemiRing>(m: usize, n: usize,x: T, mat: &mut [T]){
   for i in 0..m{
         mat[(i*n+i) as usize]=x.clone()
       }
}

pub fn permute_antidiagonal<T: SemiRing>(m: usize, n: usize,x: T, mat: &mut [T]){
       for i in 0..m{
         mat[(i*n + (n-i-1)) as usize]=x.clone()
       }
}

pub fn permute_row<T: SemiRing>(m: usize, n: usize, col: usize, x: T, mat: &mut [T]){
      for i in 0..m{
         mat[(i*n) as usize]=x.clone()
       }
}

pub fn permute_col<T: SemiRing>(m: usize, n: usize, row: usize, x: T, mat: &mut [T]){
    for i in 0..m{
         mat[(i*n) as usize]=x.clone()
       }
}

pub fn diagonal_product<T: SemiRing>(m: usize, n: usize, mat: &[T]) -> T{
        let mut k = T::default().mul_identity();
        for i in 0..m{
         k = k.product(&mat[(i*n+i) as usize])
       }
       k
}

pub fn symmetric<T: SemiRing>(m: usize, n: usize, mat: &[T]) -> bool{
      for i in 0..n{
      for j in 0..m{
        if  mat[i*m+j] !=mat[j*m+i]{
         return false
     
        }
       }
      }
     return true
     }

pub fn skewsymmetric<T: Ring>(m: usize, n: usize, mat: &[T]) -> bool{
    for i in 0..n{
          for j in 0..m{
            if  mat[i*m+j] != mat[j*m+i].add_inverse(){
              return false
             }
           }
        }
        return true
       }
/*       
pub fn diagonal<T: SemiRing>(m: usize, n: usize, mat: &[T]) -> bool{
     if m != n {return false}
     
     for i in 
}

pub fn uppertriangular<T: SemiRing>(m: usize, n: usize, mat: &[T]) -> bool{
     
}  

pub fn lowertriangular<T: SemiRing>(m: usize, n: usize, mat: &[T]) -> bool{

}
*/

pub fn rowswap<T: SemiRing>(m: usize, n: usize,row: usize, row2: usize, mat: &mut [T]){
        for i in 0..n{
           mat[..].swap(row*n + i, row2*n + i)
         }
}

pub fn colswap<T: SemiRing>(m: usize, n: usize,col: usize, col2: usize, mat: &mut [T]){
        for i in 0..m{
           mat[..].swap(i*n + col, i*n + col2)
         }
}



