
 use crate::Mpz;

use crate::traits::*;
use crate::HyperComplex;

 impl<T: Ring> MulOp for HyperComplex<T,4> {
 
   fn product(&self, other: &Self) -> Self {
      Self::unchecked_new([
      self.re[0].product(&other.re[0]).addition(&self.re[1].product(&other.re[1].add_inverse())).addition( 
      &self.re[2].product(&other.re[2].add_inverse() )).addition( &self.re[3].product(&other.re[3].add_inverse()) ),
      
      self.re[0].product(&other.re[1]).addition( &self.re[1].product(&other.re[0])).addition(  
      &self.re[2].product(&other.re[3])).addition(&self.re[3].product(&other.re[2].add_inverse())),
      
      self.re[0].product(&other.re[2]).addition( &self.re[1].product(&other.re[3].add_inverse()) ).addition( 
      &self.re[2].product(&other.re[0])).addition( &self.re[3].product(&other.re[1])),
      
      self.re[0].product(&other.re[3]).addition( &self.re[1].product(&other.re[2])).addition(
      &self.re[2].product(&other.re[1].add_inverse())).addition( &self.re[3].product(&other.re[0]))
      ])
   }
 }
 
 impl<T:  Field > MulInverse for HyperComplex<T,4>{
 
    fn mul_inverse(&self)->Self{
        let norm_inv = self.norm_sqr().mul_inverse();
       Self::unchecked_new(
       [self.re[0].product(&norm_inv) , self.re[1].add_inverse().product(&norm_inv),
       self.re[2].add_inverse().product(&norm_inv),self.re[3].add_inverse().product(&norm_inv)  ]
       )
    }
 }
 
 impl  HyperComplex<i64,4>{
     
      fn irreducible(self)->bool{
         self.norm_sqr().irreducible()
     } 
 }
 
 impl  HyperComplex<Mpz,4>{
     
      fn irreducible(self)->bool{
         self.norm_sqr().irreducible()
     } 
 }
