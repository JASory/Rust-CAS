/*
 Rational struct 
*/

use crate::traits::*;

use crate::numeric::Sign;

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
 pub struct Frac<T> where T: Euclidean{
  sign: Sign,
     p: T,
     q: T,
 }
 
 impl<T: Euclidean + Clone> Set for Frac<T>{
       fn rand()->Self{
           Self::unchecked_new(Sign::Positive,T::rand(), T::rand())
       }
       
       fn format(&self)->String{
          self.p.format() + "/"   + &self.q.format()
       }
 }
 
 impl<T: Euclidean + Clone> Magma for Frac<T>{
     fn op(&self, other: Self)->Self{
        other
     } 
 }
 
     // forces types to implement Euclidean trait
 impl<T : Euclidean + Clone>  Frac<T>{
 
    pub fn unchecked_new(sign: Sign, p: T, q: T)->Self{
        Self{sign, p, q}
    }
    
    pub fn sign(&self)->Sign{// returns sign 
        self.sign
    }
    
    pub fn numerator(&self)->T{// returns p 
        self.p.clone()
    }
    pub fn denominator(&self)->T{//returns q
        self.q.clone()
    }
 }
 
 
 impl <T: Euclidean + SemiRing + Clone> MulIdentity for Frac<T>{
     
     fn mul_identity()->Self{
         Self::unchecked_new(Sign::Positive,T::mul_identity(),T::mul_identity() )
    }
 }
 
 impl <T: Euclidean + SemiRing + Clone> MulInverse for Frac<T>{
     
     fn mul_inverse(&self)->Self{
         Self::unchecked_new(self.sign,self.q.clone(),self.p.clone() )
    }
 }
 
 impl <T: Euclidean + SemiRing + Mul<Output =T> + Clone> Mul for Frac<T>{
            type Output = Self;
      fn mul(self, other: Self)->Self{
       Self::unchecked_new(self.sign,self.p.clone()*other.p.clone(), self.q.clone()*other.q.clone())
    }
 }
 
 impl <T: Euclidean + SemiRing + Add<Output =T> + Clone> AddIdentity for Frac<T>{
     
     fn add_identity()->Self{
         Self::unchecked_new(Sign::Positive,T::add_identity(),T::add_identity() )
    }  
 } 
 
 impl <T: Euclidean + SemiRing + Add<Output =T> + Clone> AddInverse for Frac<T>{
     
     fn add_inverse(&self)->Self{
         Self::unchecked_new(self.sign.neg(),self.p.clone(),self.q.clone() )
    }  
 }
 
  impl <T: Euclidean + SemiRing + Add<Output =T> + Mul<Output =T> + Clone + PartialEq> Add for Frac<T>{
           type Output = Self;
     fn add(self, other: Self)->Self{
       if self.p == T::add_identity() ||  self.q == T::add_identity() {
           return other.clone()
       }
       if other.p == T::add_identity() ||  other.q == T::add_identity() {
         return  self.clone()
       }
         Self::unchecked_new(self.sign, self.p.clone()*other.q.clone() + other.p.clone()*self.q.clone(), self.q.clone()*other.q.clone()) 
    }  
 }
 /*
 impl<T: EuclideanDomain + SemiRing + Clone> EuclideanDomain for Frac<T>{
       
       fn remainder(&self, other: Self)->Self{
          
       }
 }
 */
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + PartialEq + Clone > AdditiveGroup for Frac<T> {}
 
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + PartialEq + Clone > MultiplicativeGroup for Frac<T> {}
  
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + PartialEq + Clone > SemiRing for Frac<T> {}
 
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + PartialEq + Clone > Ring for Frac<T> {fn characteristic()->u64 {0u64} }
   
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + PartialEq + Clone > Field for Frac<T> {}
 
 /*
 
    Rational Numbers Q
  
 */
 impl Frac<u64>{
 
 
 pub fn reduce(&mut self){
        let gcd = self.p.gcd(self.q.clone());
        self.p = self.p/gcd;
        self.q = self.q/gcd; 
 } 
 
 /*
 pub fn reduce_add()->Self{}
 
 pub fn reduce_mul()->Self{}
 */
 //                      p   a   r   s
 pub fn p_adic(&self)->(u64,u64,u64,u64){
        let mut k = self.clone();
        k.reduce();
        let factors = k.p.factor();
        let base = factors[0];
        let mut a = 0;
        for i in factors{
          if i == base{
          a+=1;
          }
        }
       // println!("{}",k.p);
        let r = k.p - base.pow(a);
        (base,a as u64,r,k.q)
        
 }
 
 }
  
 
 
