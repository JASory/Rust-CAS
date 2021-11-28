/*
 Generalized Matrix functions

*/
   use std::ops::*;
   use crate::traits::*;

 #[derive(Debug, PartialEq, Clone)]
 pub struct Matrix<T> where T: SemiRing {
                m: usize, // number of rows
                n: usize, // number of columns
   pub   elements: Vec<T>,     
 }
 
 
 impl<T: SemiRing + PartialEq + Clone> Matrix<T>{
 
  pub fn unchecked_new(m: usize, n: usize, elements: Vec<T>)->Self{
        Self{m,n,elements}
  }
  
  pub fn row_num(&self)->usize{
          self.m
  }
  
  pub fn col_num(&self)->usize{
          self.n
  }
  
  pub fn is_square(&self)->bool{
          self.m == self.n 
  }
        // check if row exists
  pub fn unchecked_row(&mut self, num: usize)->Vec<T>{
          let mut t = vec![];
         t.extend_from_slice(&mut self.elements[num*self.m..num*self.m+self.m]);
         t 
  }
         //check if col exists
  pub fn unchecked_col(&self, num: usize)->Vec<T>{
          let mut t = vec![];
            for i in 0..self.n{
              t.push(self.elements[i*self.m + num].clone())
            }
            t
  }
      // check if square
  pub fn unchecked_diagonal(&self)->Vec<T>{
       let mut t = vec![];
            for i in 0..self.m{
                t.push(self.elements[(i*self.m+i) as usize].clone())
            }
            t
  }
      //check if square
  pub fn unchecked_antidiagonal(&self)->Vec<T>{
          let mut t = vec![];
             for i in 0..self.m{
                t.push(self.elements[(i*self.m + (self.m-i-1)) as usize].clone())
            }
            t
  }
  
  /*
     Intialization functions 
  */
  
  pub fn set_rand(m:usize, n: usize)->Self{
       Self::unchecked_new(m,n,(0..m*n).map(|_| T::rand()).collect::<Vec<T>>())
  }
  
  pub fn set_identical(m: usize, n: usize, x: T)->Self{
       Self::unchecked_new(m,n,(0..m*n).map(|_| x.clone()).collect::<Vec<T>>() )
   }
  
  pub fn add_identity(m: usize, n: usize)->Self{
      Self::set_identical(m,n,T::add_identity())
   }
   
   
  pub fn set_diagonal(m :usize,x: T)->Self{
       let mut k = Self::add_identity(m,m);
       for i in 0..m{
         k.elements[(i*m+i) as usize]=x.clone()
       }
       k
  } 
  
  pub fn set_antidiagonal(m: usize, x: T)->Self{
       let mut k = Self::add_identity(m,m);
       for i in 0..m{
         k.elements[(i*m + (m-i-1)) as usize]=x.clone()
       }
       k
  }
   
  pub fn mul_identity(m: usize)->Self{
       Self::set_diagonal(m,T::mul_identity())
  }
  
  
  
 pub fn unchecked_transpose(&self)-> Matrix<T>{
       let mut tmatrix= Self::unchecked_new(self.m, self.n,vec![]);
       
         for x in 0..self.m {
           for i in 0..self.n{
              tmatrix.elements.push(self.elements[self.m * i + x].clone())
           }
         }
        tmatrix 
      }//end function
        // check if indices exist
 pub fn unchecked_rowswap(&mut self,x: usize, y: usize){
         let cols = self.col_num();
         for i in 0..cols{
           self.elements[..].swap(x*cols + i, y*cols + i)
         }
 }    
      /*
         Check conditional
      */
      
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
 
 impl<T: SemiRing + Clone> Matrix<T>{
 
 pub fn format(&self)->String{
         let mut k = Vec::new();
      for i in 0..self.m{
        k.push(self.elements[self.n*i..self.n*(i+1)].iter().map(|x| x.format()).collect::<Vec<String>>().join(" "))
      }
      "\n".to_string() + &k.join("\n")
 }
 }
