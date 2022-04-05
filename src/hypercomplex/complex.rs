/*
   Standard Complex Numbers i.e HyperComplex of dimension two
*/
use crate::traits::*;
use crate::HyperComplex;

impl<T: Ring > MulOp for HyperComplex<T,2> {

 fn product(&self, other: &Self)->Self{
    Self::unchecked_new(
    [ self.re[0].product(&other.re[0]).addition( &(self.re[1].product(&other.re[1]).add_inverse()) ),
      self.re[0].product(&other.re[1]).addition(  &self.re[1].product(&other.re[0]) )]
    )
 }
 
 }
 
 // (ac - bd) + ad + bc
 
 impl<T:  Field > MulInverse for HyperComplex<T,2>{
 
    fn mul_inverse(&self)->Self{
        let norm_inv = self.norm_sqr().mul_inverse();
       Self::unchecked_new(
       [self.re[0].product(&norm_inv) , self.re[1].add_inverse().product(&norm_inv) ]
       )
    }
 }
 
 
 impl HyperComplex<f64,2>{
 
 pub  fn arg(&self)->f64{
      self.re[1].atan2(self.re[0])
   }
  
 pub  fn to_polar(&self)->(f64,f64){
       ( self.norm(), self.arg() )
   }
   
 pub  fn from_polar(r: f64, theta: f64)->Self{
      Self::unchecked_new( [r*theta.cos(), r*theta.sin()])
   }
   
 pub  fn nth_root(&self, n: u64)->Self{
         let (r,x) = self.to_polar();
         let radius = r.powf( (n as f64).recip() );
      Self::unchecked_new( [radius* (x/ n as f64).cos(), radius* (x/n as f64).sin()] )
 }
   
 pub  fn nth_roots(&self, n: u64)->Vec<Self>{ // returns a nth root
 
         const tau : f64 = 6.2831853071795;
         
          let (r,x) = self.to_polar();
          let mut nthroots = vec![];
          let radius = r.powf( (n as f64).recip() );
          
      for i in 0..n {
          nthroots.push( 
            Self::unchecked_new( [radius* ((x + tau*i as f64)/n as f64).cos() , radius* ((x + tau*i as f64)/n as f64).sin()] )
          ) ;
      }
      nthroots
   }

  pub  fn pow(&self, pow: Self)->Self{
          let (r,x) = self.to_polar();
        Self::from_polar(r.powf(pow.re[0])* (-pow.re[1]*x).exp(), pow.re[0]*x + pow.re[1]*r.ln())
  } 
 }  
 
 
 
 /*
    Traits up to Field for Standard Complex numbers
 */
 
 impl<T:  AdditiveGroup> AdditiveGroup for HyperComplex<T,2> {} 
 impl<T:  Field > MultiplicativeGroup for HyperComplex<T,2>{}
 impl<T:  Field > SemiRing for HyperComplex<T,2>{}
 impl<T:  Field > Ring for  HyperComplex<T,2>{fn characteristic(&self)->u64 {0u64}}
 impl<T:  Field > Field for HyperComplex<T,2>{}
 
 
 impl   HyperComplex<i64,2>{
 
  pub  fn irreducible(self)->bool{
         if (self.re[0], self.re[1]) == (1,1) {return true}
         if self.re[0] == i64::default().add_identity() && self.re[1] != i64::default().add_identity(){
           if (self.re[1]%4 ==3)&self.re[1].irreducible(){
               return true
           }
           else {
              return false
           }
         }
         else if self.re[1] == i64::default().add_identity() && self.re[0] != i64::default().add_identity(){
              if (self.re[0]%4 ==3)&self.re[0].irreducible(){
               return true
           }
           else {
              return false
           }
         }
         else if  (self.norm_sqr()%4 ==3)== false && self.norm_sqr().irreducible(){
          return true
         }
         else {
            return false
         }
     }
     
   }
     
     /*
   impl  HyperComplex<Mpz,2> {
   
   
   
   }   
   */
