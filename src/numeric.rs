/*
 Standard Numerics implementations for field ring and Euclidean domain traits for u64, i64 and f64

*/

  use crate::traits::*;

#[derive(Clone, Copy,Debug, PartialEq)]
pub enum Sign{
   Positive,
   Negative,
}

 impl Sign {
   
  pub fn neg(&self)->Self{
      match self {
      Sign::Positive =>  Sign::Negative,
      Sign::Negative =>  Sign::Positive, 
      }
   }
   
 pub fn mul(&mut self, other: &Self)->Self{
       match (&self, other) {
         (&Sign::Positive, &Sign::Negative)=>  Sign::Negative,
         (&Sign::Negative, &Sign::Positive)=>  Sign::Negative,
                                         _=>   Sign::Positive,
       }
   }
 pub fn pow(&self, pow: &u64)-> Sign{
        if self == &Sign::Negative && pow%2 ==1{
           return Sign::Negative
        }
        else{
            Sign::Positive
        }
      }  
 }


 impl UFD for u64{
     
     fn irreducible(self)->bool{
     
         if self&1 == 0 && self !=2 || self==1 || ( self%3==0 && self !=3){  // checks for 1,2,and 3 cases
            return  false
         }
       let limit = ((self as f64).sqrt() as u64 +1u64)/6 +1;  // set upper bound as sqrt(p)/6 +1
         for i in 1..limit{
           if self%(6*i-1) == 0 || self%(6*i+1) == 0{// check if divisible by the 5-rough numbers up to the limit
             return false
           }                        // Higher p-rough sets have greater efficiency but diminishing returns 
         }
      return true
     }
 }



 impl EuclideanDomain for u64{
 
      fn remainder(self, other: Self)->Self{
         self%other
      }
      
     fn form(&self, x: Self, c: Self)->bool{
        let interim = (self-c)/x;
        interim*x + c == *self
     } 
      
     fn gcd(mut self, mut other: Self)->Self{
        use std::cmp::min;
        use std::mem::swap;

  
      if self == 0 {
          return other;
      } else if self == 0 {
          return other;
      }

      let i = self.trailing_zeros();  self >>= i;
      let j = self.trailing_zeros();  other >>= j;
      let k = min(i, j);
 
      loop {
         
         if self > other {
             swap(&mut self, &mut other);
         }
         other -= self;

         if other == 0 {
             return self << k;
         }
         other >>= other.trailing_zeros();
    }
  }
}

// trait implementations for u64 up to semiring 
 impl AddIdentity for u64 {
     fn add_identity()->Self{0u64}
 }
 
 impl AddOperation for u64 {
     fn addition(mut self, other: Self)->Self{self + other}
 }
 
 impl MulIdentity for u64{
     fn mul_identity()->Self{1u64}
 }
 
 impl MulOperation for u64{
     fn multiply(mut self, other: Self)->Self{self*other}
 }
 
 impl SemiRing for u64 {}
 
 
 
 
 
 
 
 /*
    Traits up to Ring for i64
 */
 
 impl AddIdentity for i64 {
     fn add_identity()->Self{0i64}
 }
 
 impl AddOperation for i64 {
     fn addition(mut self, other: Self)->Self{self + other}
 }
 
 impl AddInverse for i64 {
     fn add_inverse(&self)->Self {-self}
 }
 
 impl MulIdentity for i64{
     fn mul_identity()->Self{1i64}
 }
 
 impl MulOperation for i64{
     fn multiply(mut self, other: Self)->Self{self*other}
 }
 
 impl SemiRing for i64 {}
 
 impl Ring for i64 {}
 
 
 impl EuclideanDomain for i64{
      
      fn remainder(self, other: Self)->Self{
         (self as u64 %other as u64) as i64
      }
      
      fn form(&self, x: Self, c: Self)->bool{
        let interim = (self-c)/x;
        interim*x + c == *self
     } 
      
      fn gcd(self, other: Self)->Self{
         (self as u64).gcd(other as u64) as i64
      }
 }
 
 impl UFD for i64{
      
      fn irreducible(self)->bool{
           (self as u64).irreducible()
      }
 }
 
/*
f64 traits up to Field

*/ 
 
 impl AddIdentity for f64 {
     fn add_identity()->Self{0f64}
 }
 
 impl AddOperation for f64 {
     fn addition(mut self, other: Self)->Self{self + other}
 }
 
 impl AddInverse for f64 {
     fn add_inverse(&self)->Self {-self}
 }
 
 impl MulIdentity for f64{
     fn mul_identity()->Self{1f64}
 }
 
 impl MulOperation for f64{
     fn multiply(mut self, other: Self)->Self{self*other}
 }
 
 impl MulInverse for f64{
     fn mul_inverse(&self)->Self{self.recip()}
 }
 
 impl AdditiveGroup for f64 {}
 
 impl MultiplicativeGroup for f64 {}
 
 impl SemiRing for f64 {}
 
 impl Ring for f64 {}
 
 impl Field for f64 {}




 
