/*
  Field of Fractions extension of integral domains
*/

use crate::traits::*;

#[derive(Clone, Default, Debug, PartialEq)]
 pub struct Frac<T> {
     p: T,
     q: T,
 }
 
 impl<T: Set> Frac<T> {
 
 pub fn unchecked_new(p: T, q: T) -> Self{
      Self{p,q}
 }
 
 pub fn numerator(&self)->T{// returns p 
     self.p.clone()
 }
 
 pub fn denominator(&self)->T{//returns q
     self.q.clone()
 }
 
 }
 
 impl<T: Euclidean > Set for Frac<T>{
       fn rand(&self)->Self{
           Self::unchecked_new(T::default().rand(), T::default().rand())
       }
       
       fn format(&self)->String{
          self.p.format() + "/"   + &self.q.format()
       }
 }
 
 impl<T: Euclidean > Magma for Frac<T>{
     fn op(&self, other: &Self)->Self{
        other.clone()
     } 
 }
 
 impl <T: Euclidean > MulIdentity for Frac<T>{
     
     fn mul_identity(&self)->Self{
         Self::unchecked_new(T::default().mul_identity(),T::default().mul_identity() )
    }
 }
 
 impl <T: Euclidean > MulInverse for Frac<T>{
     
     fn mul_inverse(&self)->Self{
         Self::unchecked_new(self.q.clone(),self.p.clone() )
    }
 }
 
 impl <T: Euclidean > MulOp for Frac<T>{

      fn product(&self, other: &Self) -> Self{
       Self::unchecked_new(self.p.product(&other.p), self.q.product(&other.q))
    }
 }
 
 impl <T: Euclidean > AddIdentity for Frac<T>{
     
     fn add_identity(&self)->Self{
         Self::unchecked_new(T::default().add_identity(),T::default().add_identity() )
    }  
 } 
 
 impl <T: AddInverse + Euclidean > AddInverse for Frac<T>{
     
     fn add_inverse(&self)->Self{
         Self::unchecked_new(self.p.add_inverse(),self.q.clone().add_inverse() )
    }  
 }
 
 impl <T: Euclidean > AddOp for Frac<T>{

     fn addition(&self, other: &Self) -> Self {
       if self.p == T::default().add_identity() ||  self.q == T::default().add_identity() {
           return other.clone()
       }
       if other.p == T::default().add_identity() ||  other.q == T::default().add_identity() {
         return  self.clone()
       }
         Self::unchecked_new( self.p.product(&other.q).addition(&other.p.product(&self.q)), self.q.product(&other.q)) 
    }  
 }
 impl<T: Euclidean + Ring> SemiRing for Frac<T> {}
 
 impl<T: Euclidean + Ring> Ring for Frac<T> {fn characteristic(&self)->u64 {0u64} }

 impl<T: Euclidean + Ring> CommutativeRing for Frac<T> {}
  
 impl<T: Euclidean + Ring> Integral for Frac<T> {}
 
 impl<T: Euclidean + Ring> IntegralClosed for Frac<T> {}
 
 impl<T: Euclidean + Ring> GCD for Frac<T>{
       
       fn gcd(&self, other: &Self)->Self{
          Frac::unchecked_new( (self.p.product(&other.q) ).gcd( &self.q.product(&other.p) ),(self.q.product(&other.q)) )
       }
 }
 
 impl<T: Euclidean + Ring> AdditiveGroup for Frac<T> {}
 
 impl<T: Euclidean + Ring> MultiplicativeGroup for Frac<T> {}
   
 impl<T: Euclidean + Ring> Field for Frac<T> {}
 
