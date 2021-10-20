/*
 Rational struct 

*/

use crate::traits::*;

use crate::numeric::Sign;

use std::ops::*;

#[derive(Clone, Copy, Debug)]
 pub struct Rational<T>{
  sign: Sign,
     p: T,
     q: T,
 }
 
 impl<T> Set for Rational<T>{}
 
 impl<T> Magma for Rational<T>{
     fn op(&self, other: Self)->Self{
        other
     } 
 }
 
     // forces types to implement EuclideanDomain trait
 impl<T : Euclidean + Clone>  Rational<T>{
 
    pub fn new(sign: Sign, p: T, q: T)->Self{
        Rational{sign, p, q}
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
 
 
 impl <T: Euclidean + SemiRing + Clone> MulIdentity for Rational<T>{
     
     fn mul_identity()->Self{
         Rational::new(Sign::Positive,T::mul_identity(),T::mul_identity() )
    }
 }
 
 impl <T: Euclidean + SemiRing + Clone> MulInverse for Rational<T>{
     
     fn mul_inverse(&self)->Self{
         Rational::new(self.sign,self.q.clone(),self.p.clone() )
    }
 }
 
 impl <T: Euclidean + SemiRing + Mul<Output =T> + Clone> Mul for Rational<T>{
            type Output = Self;
      fn mul(self, other: Self)->Self{
       Rational::new(self.sign,self.p.clone()*other.p.clone(), self.q.clone()*other.q.clone())
    }
 }
 
 impl <T: Euclidean + SemiRing + Add<Output =T> + Clone> AddIdentity for Rational<T>{
     
     fn add_identity()->Self{
         Rational::new(Sign::Positive,T::add_identity(),T::add_identity() )
    }  
 } 
 
 impl <T: Euclidean + SemiRing + Add<Output =T> + Clone> AddInverse for Rational<T>{
     
     fn add_inverse(&self)->Self{
         Rational::new(self.sign.neg(),self.p.clone(),self.q.clone() )
    }  
 }
 
  impl <T: Euclidean + SemiRing + Add<Output =T> + Mul<Output =T> + Clone > Add for Rational<T>{
           type Output = Self;
     fn add(self, other: Self)->Self{
         Rational::new(self.sign, self.p.clone()*other.q.clone() + other.p.clone()*self.q.clone(), self.q.clone()*other.q.clone()) 
    }  
 }
 /*
 impl<T: EuclideanDomain + SemiRing + Clone> EuclideanDomain for Rational<T>{
       
       fn remainder(&self, other: Self)->Self{
          
       }
 }
 */
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + Clone > AdditiveGroup for Rational<T> {}
 
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + Clone > MultiplicativeGroup for Rational<T> {}
  
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + Clone > SemiRing for Rational<T> {}
 
  impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + Clone > Ring for Rational<T> {}
   
 impl<T: Euclidean +  Add<Output =T> + Mul<Output =T> + Clone > Field for Rational<T> {}
 
 
     // operator overload += 
 impl<T: Euclidean + SemiRing + Add<Output =T> + Mul<Output =T> + Clone> AddAssign for Rational<T>{
     
     fn add_assign(&mut self, other: Self){
         let k =  self.clone().add(other);
          self.p = k.p;
          self.q = k.q;
    }
 }
 
 
