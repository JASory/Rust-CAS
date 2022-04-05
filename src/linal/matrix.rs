 /*
  MxN Matrix M may equal N but is not guaranteed
 */
 
 use crate::traits::*;
 use crate::linal::blas::lvlthree::*;
 use crate::blas::lvlone::add;
 
 use crate::matrixop::*;

 #[derive(Debug, Default, PartialEq, Clone)]
 pub struct Matrix<T> where T: SemiRing {
                    m: usize, // number of rows
                    n: usize, // number of columns
  pub(crate) elements: Vec<T>,     
 }
 
 
 impl<T: SemiRing> Matrix<T>{
 
 pub fn unchecked_new(m: usize, n: usize, elements: Vec<T>) -> Self{
         Self{m,n,elements}
 }
 
 pub fn row_num(&self)->usize{
          self.m
 }
  
 pub fn col_num(&self)->usize{
          self.n
 }
 
 pub fn is_square(&self) -> bool{
     self.m == self.n
 }
 
 pub fn is_symmetric(&self) -> bool{
     if self.n != self.m {return false}
       symmetric(self.n, self.n, &self.elements)
 }
 
 pub fn format(&self) -> String{
    matrix_format(self.m,self.n,&self.elements)
 }
 
 pub fn set_identical(m: usize, n: usize, x: T)->Self{
       Self::unchecked_new(m,n,(0..m*n).map(|_| x.clone()).collect::<Vec<T>>() )
   }
   
 pub fn row_swap(&mut self, x: usize, y: usize){
       if x > self.m || y > self.m {panic!("Not a valid row")}
      rowswap(self.m,self.n,x,y,&mut self.elements);
 }
 
 pub fn col_swap(&mut self, x: usize, y: usize){
        if x > self.n || y > self.n {panic!("Not a valid column")}
      colswap(self.m,self.n,x,y,&mut self.elements);
 }  
  
 pub fn add_identity(m: usize, n: usize)->Self{
      Self::set_identical(m,n,T::default().add_identity())
   }
 
 pub fn mut_addition(&mut self, other: Self){
      if (self.m,self.n) != (other.m,other.n){panic!("Unequal dimensions")}
      add(&other.elements[..],&mut self.elements[..])
 } 
 
 pub fn mul_identity(m: usize, n: usize) -> Self{
         if m != n {panic!("No identity element")}
         let mut k = Self::add_identity(m,n);
          permute_diagonal(m,n,T::default().mul_identity(),&mut k.elements);
          k
 }
       // matrix multiplication with zero checks on dimensions
 pub fn gemm(&self, other: &Self)-> Self{
          let (row, col) = (self.row_num(), self.col_num());
          let mut z = Self::add_identity(row,col);
          unchecked_gemm(row,col,&self.elements[..],&other.elements[..], &mut z.elements[..]);
          z
 }
 
 
 pub fn kronecker_identity() -> Self{
         Matrix::unchecked_new(1,1,vec![T::default().mul_identity()])
 }
 
 pub fn kronecker_product(&self, other: &Self)->Self{
 
           let (row,col) = (self.row_num(), self.col_num());
           let mut interim = Self::unchecked_new(
           row*other.row_num(), col*other.col_num(), vec![]);
           
          for l in 0..self.row_num(){ 
           for k in 0..other.row_num(){
             for j in 0..self.col_num(){
              for i in 0..other.col_num(){
                interim.elements.push( 
                self.elements[l*self.col_num() +j].product(
                &other.elements[k*other.col_num() +i]) 
                )
              }
             } 
            }
           } 
         interim
  }
  
  pub fn kronecker_sum(&self, other: &Self) -> Self{
       if self.is_square()&other.is_square() == false{
         panic!("One or both of the matrices are non-square")
       }
       
       let rhs_identity = Self::mul_identity(other.n, other.n);
       let lhs_identity = Self::mul_identity(self.n, self.n);
       
       let mut lhs = self.kronecker_product(&rhs_identity);
       let rhs = lhs_identity.kronecker_product(&other);
       lhs.mut_addition(rhs);
       lhs
  }
  
 
  pub fn diagonal_det(&self) -> T{
      diagonal_product(self.n,self.m,&self.elements)
  }
 
 }
 
 
