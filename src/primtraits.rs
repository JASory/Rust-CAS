/*
   External types 

*/
use number_theory::Mpz;
use number_theory::Sign;

use number_theory::NumberTheory;
use crate::traits::*;


 
 fn to_string(x: &f64) -> String {
 if   x == &f64::INFINITY {
      return "∞".to_string()
  }
  if x.is_nan(){
      return "NaN".to_string()
  }
  if x == &0f64{
     return "0".to_string()
  }
  else {
      let mut num = String::new();
      let mut exponent = String::new();
      let mut Sign = String::new();
      let exp  = x.log10().floor();
      if exp >= 0.0{
          exponent = "E+".to_string()+ &exp.to_string();
      }
      
      else {
          exponent="E".to_string() + &exp.to_string();
      }
      
      
      
      num = (*x/10f64.powf(exp)).to_string();
      num.truncate(17);  // limits to 16 places
               return num + &exponent;
 }
 }
 
 // Set for primitive types

 impl Set for Mpz{
 
 fn rand(&self) -> Self {
 	let k = (0..self.len()).map(|x| u64::rng()).collect::<Vec<u64>>();
 	Mpz::new(Sign::Positive,k)
 	
 }
 
 fn format(&self) -> String {self.to_string()}
 
 }
 
 impl Set for f32{
 
 fn rand(&self) -> Self {unsafe{std::mem::transmute::<u32,f32>(u32::default().rand())} }
 fn format(&self) -> String {to_string(&(*self as f64))}
 }
 
 impl Set for f64{
 
 fn rand(&self) -> Self {unsafe{std::mem::transmute::<u64,f64>(u64::default().rand())} }
 fn format(&self) -> String {to_string(self)}
 }
 
 /*
    Trait implementing macros
 */
 
macro_rules! set_impl(
    ($($t:ty),* $(,)*) => {$(
        impl Set for $t{
            fn rand(&self) -> Self{
                <Self as NumberTheory>::rng()
            }
            
            fn format(&self) -> String{
                self.to_string()
            }
        }
         )*}
);  
 
macro_rules! addop_impl(
    ($($t:ty),* $(,)*) => {$(
       impl AddOp for $t{
            fn addition(&self, other: &Self) -> Self{
                *self + *other
            }
        }
    )*}
);

macro_rules! mulop_impl(
    ($($t:ty),* $(,)*) => {$(
       impl MulOp for $t{
            fn product(&self, other: &Self) -> Self{
                *self * *other
            }
        }
    )*}
);
 // Magma 
 

macro_rules! magma_impl(
    ($($t:ty),* $(,)*) => {$(
        impl Magma for $t{
            fn op(&self, other: &Self) -> Self{
                other.clone()
            }
        }
    )*}
);

macro_rules! addid_impl(
    ($($t:ty),* $(,)*) => {$(
        impl AddIdentity for $t{
            fn add_identity(&self) -> Self{
                0
            }
          }  
    )*}
);

macro_rules! mulid_impl(
    ($($t:ty),* $(,)*) => {$(
        impl MulIdentity for $t{
            fn mul_identity(&self) -> Self{
                1
            }
          }  
    )*}
);


macro_rules! gcd_impl {
    ($t:ident) => {
        impl GCD for $t{
            fn gcd(&self, other: &Self) -> Self{
                NumberTheory::euclid_gcd(&self, &other)
            }
        }
    }
}

macro_rules! ufd_impl {
    ($t:ident) => {
        impl UFD for $t{
            fn irreducible(&self) -> bool{
                NumberTheory::is_prime(self)
            }
            
            fn factorization(&self) -> Vec<Self>{
                NumberTheory::factor(&self)
            }
        }
    }
}

macro_rules! euclid_impl{
   ($t:ident) => {
       impl Euclidean for $t{
         fn euclidean(&self, other: &Self) -> (Self,Self){
             NumberTheory::euclidean_div(&self, &other)
         }
       }
   }
}

 set_impl!(u8,i8,u16,i16,i32,u32,i64,u64,i128,u128);
 magma_impl!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,Mpz,f32,f64);
 
 
 // Addition for primitives

 addop_impl!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,f32,f64);
 
 impl AddOp for Mpz{
   fn addition(&self, other: &Self)-> Self{
      Mpz::ref_addition(&self, &other.clone())
   }
 }
 
 // Add Identity for primitives
 
  addid_impl!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128);
 
 impl AddIdentity for Mpz { fn add_identity(&self) -> Self {Mpz::zero()}}
 impl AddIdentity for f32 { fn add_identity(&self) -> Self {0f32} }   
 impl AddIdentity for f64 { fn add_identity(&self) -> Self {0f64} }
 
 // Add Inverse for primitives
 
 impl AddInverse for i8  { fn add_inverse(&self) -> Self {-self} }
 impl AddInverse for i16 { fn add_inverse(&self) -> Self {-self} }
 impl AddInverse for i32 { fn add_inverse(&self) -> Self {-self} }
 impl AddInverse for i64 { fn add_inverse(&self) -> Self {-self} }
 impl AddInverse for i128{ fn add_inverse(&self) -> Self {-self} }
 impl AddInverse for f32 { fn add_inverse(&self) -> Self {-self} }
 impl AddInverse for f64 { fn add_inverse(&self) -> Self {-self} }
 
 impl AddInverse for Mpz { fn add_inverse(&self) -> Self {let mut k = self.clone(); k.neg(); k}}
 
 
 
 mulop_impl!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,f32,f64);
 
 impl MulOp for Mpz{
   fn product(&self, other: &Self)-> Self{
      Mpz::ref_product(&self, &other)
   }
 }
 
  // Mul Identity for primitives
  
  mulid_impl!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128);
 
 impl MulIdentity for Mpz { fn mul_identity(&self) -> Self {Mpz::one()} }
 impl MulIdentity for f32 { fn mul_identity(&self) -> Self {1f32} }
 impl MulIdentity for f64 { fn mul_identity(&self) -> Self {1f64} }
 
 
 
 
 
 

 
 // Multiplicative inverse 
 
 impl MulInverse for f32{ fn mul_inverse(&self) -> Self {self.recip()} }
 impl MulInverse for f64{ fn mul_inverse(&self) -> Self {self.recip()} }
 
 
 //   Potentially move to general file that derives from all datatypes that implement
 impl AdditiveGroup for i8  {}
 impl AdditiveGroup for i16 {}
 impl AdditiveGroup for i32 {}
 impl AdditiveGroup for i64 {}
 impl AdditiveGroup for i128{}
 impl AdditiveGroup for Mpz {}
 
 impl MultiplicativeGroup for f32{}
 impl MultiplicativeGroup for f64{}
 
 
 // SemiRing for 
 impl SemiRing for u8{}
 impl SemiRing for i8{}
 impl SemiRing for u16{}
 impl SemiRing for i16{}
 impl SemiRing for u32{}
 impl SemiRing for i32{}
 impl SemiRing for u64{}
 impl SemiRing for i64{}
 impl SemiRing for u128{}
 impl SemiRing for i128{}
 
 impl SemiRing for Mpz{}
 impl SemiRing for f32{}
 impl SemiRing for f64{}
 
 impl Ring for i8  {fn characteristic(&self) -> u64 {0u64}}
 impl Ring for i16 {fn characteristic(&self) -> u64 {0u64}}
 impl Ring for i32 {fn characteristic(&self) -> u64 {0u64}}
 impl Ring for i64 {fn characteristic(&self) -> u64 {0u64}}
 impl Ring for i128{fn characteristic(&self) -> u64 {0u64}}
 impl Ring for f32 {fn characteristic(&self) -> u64 {0u64}}
 impl Ring for f64 {fn characteristic(&self) -> u64 {0u64}}
 impl Ring for Mpz {fn characteristic(&self) -> u64 {0u64}}
 
 impl CommutativeRing for i8  {}
 impl CommutativeRing for i16 {}
 impl CommutativeRing for i32 {}
 impl CommutativeRing for i64 {}
 impl CommutativeRing for i128{}
 impl CommutativeRing for f32 {}
 impl CommutativeRing for f64 {}
 impl CommutativeRing for Mpz {}

 impl Integral for i8   {}
 impl Integral for i16  {}
 impl Integral for i32  {}
 impl Integral for i64  {}
 impl Integral for i128 {}
 impl Integral for Mpz  {}
 impl Integral for f32  {}
 impl Integral for f64  {}
 
 

 impl IntegralClosed for i8   {}
 impl IntegralClosed for i16  {}
 impl IntegralClosed for i32  {}
 impl IntegralClosed for i64  {}
 impl IntegralClosed for i128 {}
 impl IntegralClosed for Mpz  {}
 impl IntegralClosed for f32  {}
 impl IntegralClosed for f64  {}
 
 
 // Implement GCD 

 gcd_impl!(i8);
 gcd_impl!(i16);
 gcd_impl!(i32);
 gcd_impl!(i64);
 gcd_impl!(i128);
 
 impl GCD for Mpz{
   fn gcd(&self, other: &Self) -> Self{
      NumberTheory::euclid_gcd(self,other)
   }
  }
 
 impl GCD for f32{
  fn gcd(&self, other: &Self) -> Self{
     NumberTheory::euclid_gcd(&(self.clone() as u64),&(*other as u64)) as f32
  }
  }
  
  impl GCD for f64{
  fn gcd(&self, other: &Self) -> Self{
     NumberTheory::euclid_gcd(&(self.clone() as u64),&(*other as u64)) as f64
  }
  }
  
 
 // UFD 
 ufd_impl!(i8);
 ufd_impl!(i16);
 ufd_impl!(i32);
 ufd_impl!(i64);
 ufd_impl!(i128);
 
 impl UFD for Mpz{
     fn irreducible(&self) -> bool {
          NumberTheory::is_prime(self)
     }
     fn factorization(&self) -> Vec<Self> {
         NumberTheory::factor(self)
     }
 }
 
 impl UFD for f32{
     fn irreducible(&self)->bool{false}
     fn factorization(&self)->Vec<Self>{vec![1f32]}
 }
 
 impl UFD for f64{
     fn irreducible(&self)->bool{false}
     fn factorization(&self)->Vec<Self>{vec![1f64]}
 }

 impl PID for i8   {}
 impl PID for i16  {}
 impl PID for i32  {}
 impl PID for i64  {}
 impl PID for i128 {}
 impl PID for Mpz  {}
 impl PID for f32  {}
 impl PID for f64  {}


 euclid_impl!(i8);

 euclid_impl!(i16);
 euclid_impl!(i32);
 euclid_impl!(i64);
 euclid_impl!(i128);
 
 impl Euclidean for Mpz{
   fn euclidean(&self, other: &Self) -> (Self,Self){
       NumberTheory::euclidean_div(&self, &other)
   }
 }
 

 
 impl Field for f32 {}
 impl Field for f64 {}

 
 


 
 
