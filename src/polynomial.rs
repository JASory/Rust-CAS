use crate::traits::*;
use crate::blas1::*;
use std::ops::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Univariate<T>{
    coef: Vec<T>,
}

impl<T:Clone> Univariate<T>{

 pub fn unchecked_new(coef: Vec<T>)->Self{
         Self{coef}
 }
 
 pub fn lc(&self)->T{
         self.coef[self.coef.len()-1].clone()
 }
 
 
 pub fn degree(&self)->usize{
        self.coef.len()
 }

}


impl<T: Set + AddIdentity + PartialEq + Clone> Set for Univariate<T>{

  fn rand()->Self{
      Self::unchecked_new(vec![T::rand();3])
  }
  
  fn format(&self)->String{
      let limit = self.coef.len()-1;
      let  mut p = Vec::new();
      
      for i in 0..self.coef.len(){
       if self.coef[limit-i] != T::add_identity(){
        p.push(self.coef[limit-i].format() + "x^" + &((limit-i) as u64).format())
       }
      }
      p.join(" + ")
  }
}

impl<T: Set + AddIdentity + PartialEq + Clone> Magma for Univariate<T>{

 fn op(&self, other: Self)->Self{
      other
 }

}

impl<T: AddIdentity + PartialEq + Clone> AddIdentity for Univariate<T>{

 fn add_identity()->Self{
     Self::unchecked_new(vec![T::add_identity()])
 }

}

impl<T: AddInverse + AddIdentity + PartialEq + Clone> AddInverse for Univariate<T>{

 fn add_inverse(&self)->Self{
     let mut k = self.clone();
     
     for i in k.coef.iter_mut(){
        *i= i.add_inverse();
     }
     k.clone()
 }

}

impl<T: Add<Output = T> + PartialEq + Clone> Add for Univariate<T>{
       type Output = Self;
 fn add(self, other: Self)->Self{
      let mut k = self.clone();
      add(&other.coef[..],&mut k.coef[..]);
      if k.degree() < other.degree(){
         k.coef.extend_from_slice(&other.coef[k.degree()..]);
      }
      k
  }

}

impl<T: MulIdentity + PartialEq + AddIdentity + Clone> MulIdentity for Univariate<T>{

 fn mul_identity()->Self{
     Self::unchecked_new(vec![T::mul_identity()])
 }

}

impl<T: Mul<Output =T> + AddIdentity + PartialEq  + Add<Output = T> + Clone> Mul for Univariate<T>{
     type Output = Self;
   
  fn mul(self, other: Self)->Self{
        let klen = self.coef.len()+other.coef.len()-1;
        let mut k = vec![T::add_identity();klen];
        for i in 0..other.coef.len(){
            for j in 0..self.coef.len(){
                k[i+j]= k[i+j].clone() +other.coef[i].clone()*self.coef[j].clone();
            }
        }
        Univariate::unchecked_new(k)
    }
    
  }
  
impl<T: Ring + Euclidean + Clone > Univariate<T>{  
 pub fn monic_euclidean(&mut self, other: Self)->(Self,Self){
          let delta = self.degree()-other.degree();
          let mut quo = vec![T::add_identity();delta+1];
          for i in 0..delta+1{
             let val = (self.coef[self.degree()-i-1].clone()).euclidean(  other.lc().clone() ).0;
             sub_scale(val.clone(),&other.coef[..],&mut self.coef[(delta-i)..]);
             quo[delta-i]=val;
          }
         let t=  Univariate::unchecked_new(quo);
          (t, self.clone())
    }
}   





