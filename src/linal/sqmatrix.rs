/*
    SqMatrix
*/
 use crate::traits::*;
 use crate::blas::lvlone::add;
 
 use crate::blas::lvlthree::unchecked_gemm;
 use crate::matrixop::*;
use crate::Matrix;

 #[derive(Clone,PartialEq,Debug)]
 pub struct SqMatrix<T,const M : usize>{
    pub(crate) elements: [T;M]
 }
 
 impl<T: SemiRing, const M : usize> SqMatrix<T,M>{
 
 pub fn unchecked_new(elements: [T;M]) -> Self{
      Self{elements}
 }
 
 pub fn new(elements: [T;M]) -> Option<Self>{
       if ((M as f64).sqrt().floor() * (M as f64).sqrt().floor()) as usize == M {
         return Some(Self::unchecked_new(elements))
        }
        None
 }
 
 pub fn unchecked_from_vec(x: Vec<T>)->Self{
         let one  :  [T; M] = x.try_into().unwrap_or_else(|_| panic!("vector is wrong length"));
          Self::unchecked_new(one)
    }
 
 pub fn row_num(&self) -> usize{
         (M as f64).sqrt() as usize
 }
 
 pub fn col_num(&self) -> usize{
       (M as f64).sqrt() as usize
 }
 
 pub fn zero() -> Self{
       if ((M as f64).sqrt().floor() * (M as f64).sqrt().floor()) as usize == M {
         let x = (0..M).map(|_| T::default().add_identity().clone() ).collect::<Vec<T>>();
         let one  :  [T; M] = x.try_into().unwrap_or_else(|_| panic!("vector is wrong length"));
           return Self::unchecked_new(one)
        }
        panic!("Not a perfect square")
 }
    // inplace addition
 pub fn mut_addition(&mut self, other: &Self){
         add(&other.elements[..],&mut self.elements[..])
 }
 
 pub fn set_identical(&mut self, x: T){
       for i in self.elements.iter_mut(){
         *i= x.clone();
       }
   }
   
 pub fn set_diagonal(&mut self,x: T){
        let m = self.col_num();
       for i in 0..m{
         self.elements[(i*m+i) as usize]=x.clone()
       }
  }   
  
  pub fn set_antidiagonal(&mut self,x: T){
        let m = self.col_num();
       for i in 0..m{
         self.elements[(i*m + (m-i-1)) as usize]=x.clone()
       }
  }   
                          
  pub fn is_symmetric(&self)->bool{
     
     symmetric(M,M,&self.elements)
     }
     
 pub fn row_swap(&mut self, x: usize, y: usize){
         let len = (M as f64).sqrt() as usize;
       if x > len || y > len {panic!("Not a valid row")}
      rowswap(len,len,x,y,&mut self.elements);
 }
 
 pub fn col_swap(&mut self, x: usize, y: usize){
          let len = (M as f64).sqrt() as usize;
       if x > len || y > len {panic!("Not a valid row")}
      colswap(len,len,x,y,&mut self.elements);
 }  
 
 
 pub fn trace(&self) -> T{
    let m = self.row_num();
    let mut k = T::default()::add_identity();
     for i in 0..m{
         k = k.addition(&mat[(i*n+i) as usize])
       }
       k
 }
          
 }                        
 
 
  impl<T: Ring, const M: usize> SqMatrix<T,M>{
  
  pub fn is_skew_symmetric(&self)->bool{
        skewsymmetric(M,M,&self.elements)
       }
  
 }
 
 
 
 
 impl<T: SemiRing , const M: usize> Default  for SqMatrix<T,M>{
 
  fn default() -> Self{
  
   let x = (0..M).map(|_| T::default().clone() ).collect::<Vec<T>>();
    let one  :  [T; M] = x.try_into().unwrap_or_else(|_| panic!("vector is wrong length"));
        Self::unchecked_new(one)
    
    }
 }
 
 impl<T: SemiRing, const M: usize> Set for SqMatrix<T,M>{
      fn rand(&self)->Self{
          let mut k = vec![];
            for i in 0..M{
               k.push(T::default().rand())
             }
          Self::unchecked_from_vec(k)
      }
      
      fn format(&self)->String{
          let dimen = (M as f64).sqrt() as usize;
        matrix_format(dimen,dimen,&self.elements)
      }
 }
 
 impl<T: SemiRing, const M : usize> Magma for  SqMatrix<T,M> {
 
   fn op(&self, other: &Self) -> Self{
        other.clone()
   }
 }    
 
 impl<T: SemiRing, const M : usize> AddIdentity for SqMatrix<T,M> {
 
   fn add_identity(&self) -> Self {
       Self::zero()
   }
 
 }
 
  impl<T: SemiRing, const M : usize> AddOp for SqMatrix<T,M> {
  
   fn addition(&self, other: &Self) -> Self {
   
       let mut t = self.clone();
       
       t.mut_addition(&other);
       t
   }
 
 }
 
 impl<T: Ring, const M : usize> AddInverse for SqMatrix<T,M> {
  
   fn add_inverse(&self) -> Self {
   
       let mut t = self.clone();
        for i in t.elements.iter_mut(){
          *i = i.add_inverse();
        }
       t
   }
 
 }
 
 impl<T: SemiRing, const M : usize>  SqMatrix<T,M> {
  
   pub fn hadamard_identity(&self) -> Self {
   
       let mut t = self.clone();
        for i in t.elements.iter_mut(){
          *i = i.mul_identity();
        }
       t
   }
 
 }
 
 impl<T: Field, const M : usize>  SqMatrix<T,M> {
  
   pub fn hadamard_inverse(&self) -> Self {
   
       let mut t = self.clone();
        for i in t.elements.iter_mut(){
          *i = i.mul_inverse();
        }
       t
   }
 
 }
 
 impl<T: Ring, const M : usize> MulIdentity for SqMatrix<T,M> {
  
   fn mul_identity(&self) -> Self {
        let mut k = Self::zero();
        k.set_diagonal(T::default().mul_identity());
        k
      }
 
 }
 
 
 impl<T: Ring, const M : usize> MulOp for SqMatrix<T,M> {
  
   fn product(&self, other: &Self) -> Self {
        let mut k = Self::zero();
        let m = self.col_num();
        unchecked_gemm(m,m,&self.elements[..],&other.elements[..], &mut k.elements[..]);
        k
      }
 
 }
 
  impl<T: Ring, const M : usize> SqMatrix<T,M> {
  
  
  pub fn to_matrix(&self) -> Matrix<T>{
      let len = (M as f64).sqrt() as usize;
      let mut k = vec![];
      k.extend_from_slice(&self.elements);
      Matrix::unchecked_new(len,len,k)
  }
  
   // Repeated GEMM 
   pub fn pow(&self, p: u64) -> Self {
   
  let mut z = self.mul_identity();
  let mut base = self.clone() ;
  let mut pow = p;
  if pow ==0 {
    return z 
  }

 while pow > 1 {
  
   if pow%2 == 0 {
      base = base.product(&base);
      pow>>=1;
   }
  
  else{
  
   z = z.product(&base);
   base = base.product(&base);
   pow=(pow-1)>>1;  
   
 }
 }

 base.product(&z)

   
   }
   
  }
  

 
 
 
