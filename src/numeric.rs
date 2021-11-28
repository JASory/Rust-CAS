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
 
 /*
 
 Precursor functions for Number Theorectic function
 
 */
 
 const SMALL_PRIMES : [u64;33] = [
        3,5,7,11,13,17,19,23,29,31,37,41,
        43,47,53,59,61,67,71,73,79,83,89,
        97,101, 103,107,109, 113, 127, 131,
        139, 149];
 
 fn rand()-> u64{
    let mut x: u64 = 0; 
    let k = unsafe { core::arch::x86_64::_rdrand64_step(&mut x) } ;
   x
 }
 
    // difference
 fn delta_u64(x: u64, y: u64)->u64{
      if x > y {
          x-y
       }
      else {
          y -x
      }
    }
    
   fn mod_sqr(x: u64, n: u64)->u64{
    ((x as u128 * x as u128 + 1 )%n as u128) as u64
   }
   
   // modular exponentiation
fn modpow(x : u64,mut  pow: u64, modulus: u64)-> u64{  //upgrades to u128 to allow

  let mut z = 1u128;
  let mut base = x.clone() as u128;
  let n = modulus as u128;
  if pow ==0 {
    return z as u64
  }

 while pow > 1 {
  
   if pow%2 == 0 {
      base = base*base % n ;
      pow>>=1;
   }
  
  else{
  
   z = base*z % n;
   base = base*base % n;
   pow=(pow-1)>>1;  
   
 }
 }

  (base*z % n) as u64

}

fn fermat_test(p: u64, base: u64)->bool{// fermat test
     if modpow(base,p-1, p)==1{  // if 2^p-1 mod p = 1 return true as it is a pseudoprime to base
      return  true                      
     }  
       false            // else return false
    }
    
   
 fn strong_fermat(p: u64, base: u64)->bool{// checks if base^p = 1 mod p  or base^(d*2^n)= -1 for some n  
     let zeroes = (p-1).trailing_zeros() as u64; // Breaks number down to p= d*2^n -1
     let d = (p-1)/ (1<<zeroes);
     let mut x = modpow(base,d, p); // base^d mod p
     if x == 1u64 || x==p-1{   // checks if base^p = 1 mod p  or base^(d*2^n)= -1
       return true
       }
    for _ in 0..zeroes-1{// checks for all d*2^zeroes. One is subtracted since d*2^n was already checked above
     x = modpow(x, 2, p);
     if x == p-1 {       // if any d*2^zeroes = p-1  then it passes
       return true
     }
    }
    return false        // otherwise it fails
 }
 
 fn miller_rabin(p: u64)->bool{// probabilistic miller rabin (1/4)^5 , skips 2 and 3
    for _ in 0..5{
     if strong_fermat(p,2 + rand()%(p-4)) == false{    // there is no rand function here, write your own or copy from the RNG.rs file is this repository
      return false
     }
    }
    return true
 }


   
   fn det_miller_rabin(p: u64)->bool{
       const BASE : [u64;12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
       for i in BASE{
         if p == i{
           return true
         }
         if strong_fermat(p,i)==false{
          return false
         }
       }
       return true
   }
   
   fn rho(n: u64)->u64{

  let mut x = 2; let mut y = 2; let mut d = 1;
  
  while d == 1 {
  x = mod_sqr(x,n);
  y = mod_sqr(mod_sqr(y,n),n)%n;
  d = delta_u64(x,y).gcd(n)
   }
   d
}


      
 
 
 
 
 /*
    trait implementations for u64 up to semiring 

 */
 
 impl Set for u64{
      fn rand()->Self{
          let mut x: u64 = 0; 
          let k = unsafe { core::arch::x86_64::_rdrand64_step(&mut x) } ;
          x
      }
      
      fn format(&self)->String{
           self.to_string()
      }
 }
 
 impl Magma for u64{
    fn op(&self, other: Self)->Self{other}
 }
 
 impl AddIdentity for u64 {
     fn add_identity()->Self{0u64}
 }

 impl MulIdentity for u64{
     fn mul_identity()->Self{1u64}
 }
 
 impl SemiRing for u64 {}

 
 impl GCD for u64{
 
      fn gcd(&self, mut other: Self)->Self{
        use std::cmp::min;
        use std::mem::swap;
      
      let mut v = self.clone();
    
    if other == 0 {
          return v;
      } else if v == 0 {
          return other;
      }

      let i = other.trailing_zeros();  other >>= i;
      let j = v.trailing_zeros();  v >>= j;
      let k = min(i, j);
 
      loop {
         
         if other > v {
             swap(&mut other, &mut v);
         }
         v -= other;

         if v == 0 {
             return other << k;
         }
         v >>= v.trailing_zeros();
  }
 }
 }

 impl UFD for u64{
     
     fn irreducible(self)->bool{
     
          if self&1 == 0 && self !=2 || self==1 || ( self%3==0 && self !=3){ return false} // checks for 1,2,and 3 cases
          if self < 5 && (self == 2 || self == 3 ){return true}
          for i in SMALL_PRIMES[1..].iter(){
            if self == *i{
              return true
            }
            if self%i == 0{
             return false
            }
          } 
          det_miller_rabin(self)  
     }
     
 
  fn factor(self)->Vec<u64>{
      
       let mut n = self.clone();
       let twofactors = n.trailing_zeros();
       n>>=twofactors; 
       
       let mut factors : Vec<u64> = vec![];
       
       if twofactors > 0{
          factors.push(2);
          factors.push(twofactors as u64);
       }
       
       if n.irreducible(){
        factors.push(n);
        factors.push(1);
         return factors
     }
       
       for i in SMALL_PRIMES{ // strips out small primes
          if n%i==0{
            factors.push(i);
            let mut count = 0u64;
          while n%i==0 {
            count +=1;
            n/=i;
          }
          factors.push(count);
          }
       }
       
       if n == 1 {return factors}
       
    while n != 1{
          let k = rho(n);
           factors.push(k);
           let mut count = 0u64;
      while n%k == 0{
             count+=1;
             n/=k;
           }
           factors.push(count);
       }
       factors
  }
 
 }
 
 



 impl Euclidean for u64{
 
      fn euclidean(self, other: Self)->(Self,Self){
        (self/other, self%other)
      }
      
     fn form(&self, x: Self, c: Self)->bool{
        let interim = (self-c)/x;
        interim*x + c == *self
     } 
      
}


 
 
 
 
 
 
 
 /*
    Traits up to Ring for i64
 */
 
 impl Set for i64{
     fn rand()->i64{unsafe{std::mem::transmute::<u64,i64>(u64::rand())}}
     fn format(&self)->String{self.to_string()}
 }
 
 impl Magma for i64{
    fn op(&self, other: Self)->Self{other}
 }
 
 impl AddIdentity for i64 {
     fn add_identity()->Self{0i64}
 }
 
 impl AddInverse for i64 {
     fn add_inverse(&self)->Self {-self}
 }
 
 impl MulIdentity for i64{
     fn mul_identity()->Self{1i64}
 }
 
 
 
 impl SemiRing for i64 {}
 
 impl Ring for i64 {
   fn characteristic()->u64{ 0u64}
 }
 
 
 impl GCD for i64{
      
      fn gcd(&self, other: Self)->Self{
         (self.clone() as u64).gcd(other as u64) as i64
      }
 }
 
 impl UFD for i64{
      
      fn irreducible(self)->bool{
           (self as u64).irreducible()
      }
      fn factor(self)->Vec<i64>{
         (self as u64).factor().iter().map(|x| *x as i64).collect::<Vec<i64>>()
      }
 }
 
 impl Euclidean for i64{
      
      fn euclidean(self, other: Self)->(Self,Self){
         (self/other, self%other)
      }
      
      fn form(&self, x: Self, c: Self)->bool{
        let interim = (self-c)/x;
        interim*x + c == *self
     } 
 }
 
/*
f64 traits up to Field

*/ 

 impl Set for f64{
    fn rand()->f64{unsafe{std::mem::transmute::<u64,f64>(u64::rand())} }
    fn format(&self)->String{
    
    if self == &f64::INFINITY {
      return "∞".to_string()
  }
  if self.is_nan(){
      return "NaN".to_string()
  }
  else {
      let mut num = String::new();
      let mut exponent = String::new();
      let mut Sign = String::new();
      let exp  = self.log10().floor();
      if exp >= 0.0{
          exponent = "E+".to_string()+ &exp.to_string();
      }
      
      else {
          exponent="E".to_string() + &exp.to_string();
      }
      
      
      
      num = (self/10f64.powf(exp)).to_string();
      num.truncate(17);  // limits to 16 places
               return num + &exponent;
       }
 }
 
 } // end impl 

 impl Magma for f64{
    fn op(&self, other: Self)->Self{other}
 }
 
 impl AddIdentity for f64 {
     fn add_identity()->Self{0f64}
 }
 
 
 
 impl AddInverse for f64 {
     fn add_inverse(&self)->Self {-self}
 }
 
 impl MulIdentity for f64{
     fn mul_identity()->Self{1f64}
 }
 
 
 impl MulInverse for f64{
     fn mul_inverse(&self)->Self{self.recip()}
 }
 
 impl AdditiveGroup for f64 {}
 
 impl MultiplicativeGroup for f64 {}
 
 impl SemiRing for f64 {}
 
 impl GCD for f64{// GCD floor to integer
    
    fn gcd(&self, other: Self)->Self{
        (self.clone() as u64).gcd(other as u64) as f64
    }
 }
 
 impl UFD for f64{
     
     fn irreducible(self)->bool{false}
     fn factor(self)->Vec<f64>{vec![1f64]}
 }
 
 impl Euclidean for f64{
    
    fn euclidean(self, other: Self)->(Self, Self){
       ((self/other).floor(), self - other*(self/other).floor())
    }
    
    fn form(&self, x: Self, c: Self)->bool{
        let interim = (self-c)/x;
        interim*x + c == *self
     } 
 }
 
 impl Ring for f64 {fn characteristic()->u64{ 0u64}}
 
 impl Field for f64 {}




 
