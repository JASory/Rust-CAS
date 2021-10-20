/*
 Generalized Matrix functions

*/
   use std::ops::*;
   use crate::traits::*;

 #[derive(Debug, PartialEq, Clone)]
 pub struct Matrix<T> where T: SemiRing {
             m: usize, 
             n: usize, 
   pub   elements: Vec<T>,     
 }
 
 
 impl<T: SemiRing + PartialEq + Clone> Matrix<T>{
 
  pub fn new(m: usize, n: usize, elements: Vec<T>)->Self{
        Self{m,n,elements}
  }
  
  pub fn row_len(&self)->usize{
          self.m
  }
  
  pub fn col_len(&self)->usize{
          self.n
  }
  
  pub fn is_square(&self)->bool{
          self.m == self.n 
  }
  
  pub fn unchecked_row(&mut self, num: usize)->Vec<T>{
          let mut t = vec![];
         t.extend_from_slice(&mut self.elements[num*self.m..num*self.m+self.m]);
         t 
  }
  
  pub fn unchecked_col(&self, num: usize)->Vec<T>{
          let mut t = vec![];
            for i in 0..self.n{
              t.push(self.elements[i*self.m + num].clone())
            }
            t
  }
  
      
  pub fn unchecked_ascend_diagonal(&self)->Vec<T>{
          let mut t = vec![];
             for i in 0..self.m{
                t.push(self.elements[(i*self.m + (self.m-i-1)) as usize].clone())
            }
            t
  }
  
  pub fn unchecked_descend_diagonal(&self)->Vec<T>{
       let mut t = vec![];
            for i in 0..self.m{
                t.push(self.elements[(i*self.m+i) as usize].clone())
            }
            t
  }
  
  pub fn add_identity(m: usize, n: usize)->Self{
       Self::new(m,n,(0..m*n).map(|_| T::add_identity()).collect::<Vec<T>>() )
   }
   
  pub fn mul_identity(m: usize)->Self{
       let mut k = Self::add_identity(m,m);
       for i in 0..m{
         k.elements[(i*m+i) as usize]=T::mul_identity()
       }
       k
  }
  
 pub fn unchecked_transpose(&self)-> Matrix<T>{
       let mut tmatrix= Self::new(self.m, self.n,vec![]);
       
         for x in 0..self.m {
           for i in 0..self.n{
              tmatrix.elements.push(self.elements[self.m * i + x].clone())
           }
         }
        tmatrix 
      }//end function
      
      
 pub fn is_symmetric(&self)->bool{
        if !self.is_square(){
             return false
        }
     
     for i in 0..self.m{
      for j in 0..self.m{
        if  self.elements[i*self.m+j] !=self.elements[j*self.m+i]{
         return false
     
        }
       }
      }
     return true
     }     
 }
 
 impl<T: SemiRing + PartialEq + AddInverse + Clone> Matrix<T>{
  pub fn is_skew_symmetric(&self)->bool{
   
        if !self.is_square(){
              return false
            }
     
       for i in 0..self.m{
          for j in 0..self.m{
            if  self.elements[i*self.m+j] != self.elements[j*self.m+i].add_inverse(){
              return false
             }
           }
        }
        return true
       }
 
 }
