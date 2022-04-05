

use crate::traits::*;
use crate::blas::lvlone::*;

use std::ops::*;


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Univariate<T>{
    coef: Vec<T>,
}


impl<T:Set> Univariate<T>{

 pub fn unchecked_new(coef: Vec<T>)->Self{
         Self{coef}
 }
 
 pub fn lc(&self) -> T{
         self.coef[self.coef.len()-1].clone()
 }
 
 
 pub fn degree(&self)->usize{
        self.coef.len()
 }

}


impl<T: Ring> Set for Univariate<T>{

  fn rand(&self) -> Self{
      Self::unchecked_new(vec![T::default().rand();3])
  }
  
  fn format(&self) -> String{
      let limit = self.coef.len()-1;
      let  mut p = Vec::new();
      
      for i in 0..self.coef.len(){
       if self.coef[limit-i] != T::default().add_identity(){
        p.push(self.coef[limit-i].format() + "x^" + &((limit-i) as u64).format())
       }
      }
      p.join(" + ")
  }
}

impl<T: Ring> Magma for Univariate<T>{

 fn op(&self, other: &Self)->Self{
      other.clone()
 }

}

impl<T:  Ring > AddIdentity for Univariate<T>{

 fn add_identity(&self)->Self{
     Self::unchecked_new(vec![T::default().add_identity()])
 }

}

impl<T: Ring > AddInverse for Univariate<T>{

 fn add_inverse(&self)->Self{
     let mut k = self.clone();
     
     for i in k.coef.iter_mut(){
        *i= i.add_inverse();
     }
     k.clone()
 }

}

impl<T: Ring> AddOp for Univariate<T>{

 fn addition(&self, other: &Self) -> Self {
      let mut k = self.clone();
      add(&other.coef[..],&mut k.coef[..]);
      if k.degree() < other.degree(){
         k.coef.extend_from_slice(&other.coef[k.degree()..]);
      }
      k
  }

}

impl<T: Ring> MulIdentity for Univariate<T>{

 fn mul_identity(&self) -> Self {
     Self::unchecked_new(vec![T::default().mul_identity()])
 }

}

impl<T: Ring> MulOp for Univariate<T>{
     
   
  fn product(&self, other: &Self) -> Self {
  
        let klen = self.coef.len()+other.coef.len()-1;
        let mut k = vec![T::default().add_identity();klen];
        
        for i in 0..other.coef.len(){
            for j in 0..self.coef.len(){
                k[i+j]= k[i+j].addition( &other.coef[i].product(&self.coef[j]) );
            }
        }
        
        Univariate::unchecked_new(k)
    }
    
  }
  
impl<T: Ring + Euclidean > Univariate<T>{ 
 
 pub fn monic_euclidean(&mut self, other: Self) -> (Self,Self) {
          let delta = self.degree()-other.degree();
          let mut quo = vec![T::default().add_identity();delta+1];

          for i in 0..delta+1{
             let val = self.coef[self.degree()-i-1].euclidean(  &other.lc() ).0;

             sub_scale(val.clone(),&other.coef[..],&mut self.coef[(delta-i)..]);
             quo[delta-i]=val;
          }
         let t=  Univariate::unchecked_new(quo);
          (t, self.clone())
    }
}

impl<T: Ring + Euclidean>Univariate<T>{
  // content and primitive polynomial
 pub fn cont_prim(&mut self)->T{
         gcd_reduce(&mut self.coef[..])
 }

}   






