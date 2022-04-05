/*
  Dual numbers. Complex analogs where i^2 = 0
  Note that dual analogs of Real-valued unary functions R can be constructed by R(a), R'(a)b
  
  Due to this, one can compute the derivatives of the functions
  
  Ex. sqrt(x) sqrt(x)' evaluated at x = 5
    Dual::new(5,1).sqrt()
  
*/
 
 use crate::traits::*;
 

#[derive(Clone,Default,PartialEq,Debug)]
 pub struct Dual{
    a: f64,
    b: f64,
 }
 
 impl Set for Dual{
 
   fn rand(&self) -> Self{
       Self::new(f64::default().rand(),f64::default().rand())
   }
   
   fn format(&self) -> String{
      self.a.format() + " + " + &self.b.format() + "ε"
   }
 
 }
 
 impl Magma for Dual{
   fn op(&self, other: &Self) -> Self{
       return other.clone()
   }
 }
 
 impl AddIdentity for Dual{
     fn add_identity(&self) -> Self{
         Self::new(self.a.add_identity(), self.b.add_identity())
     }
 }
 
 impl AddInverse for Dual{
    fn add_inverse(&self) -> Self{
       Self::new(self.a.add_inverse(),self.b.add_inverse())
    }
 }
 
 impl AddOp for Dual{
   fn addition(&self, other: &Self) -> Self{
       Self::new(self.a + other.a, self.b + other.b)
   }
 }
 
 impl MulIdentity for Dual{
   fn mul_identity(&self) -> Self{
       Self::new(self.a.mul_identity(), self.b.add_identity())
   }
 }
 
 impl MulOp for Dual{
    fn product(&self, other: &Self) -> Self{
       Self::new(self.a*other.a, self.a*other.b + other.a*self.b)
    }
 }
 
 impl Dual {
 
 pub fn new(a: f64, b: f64) -> Self{
   Self{a,b}
 }

 pub fn coef(&self) -> (f64,f64){
   (self.a,self.b)
 }
 
 pub fn sqr(&self) -> Self{
  self.product(&self)
 }

 pub fn pow(&self, n: f64) -> Self{
    Self::new(self.a.powf(n), self.b*n*self.a.powf(n-1f64))
 }

 pub fn division(&self, other: Self) -> Self{
     Self::new(self.a/other.a, (self.b*other.a - other.b*self.a)/other.a*other.a)
 }

 pub fn sqrt(&self) -> Self{
    Self::new(self.a.sqrt(), self.b/(2f64*self.a.sqrt()))
 }

 pub fn nth_rt(&self, n: u32) -> Self{
    let pow = (n as f64).recip();
    Self::new(self.a.powf(pow), self.b*pow*self.a.powf(pow-1f64))
 }

 pub fn exp(&self) -> Self{
    let e = 2.718281828f64;
    Self::new(e.powf(self.a), self.b*e.powf(self.a))
 }
   
 pub fn ln(&self) -> Self{
     Self::new(self.a.ln(), self.b/self.a)
 }
 
 pub fn cos(&self) -> Self{
    Self::new(self.a.cos(),self.b*(-self.a.sin()))
 }

 pub fn sin(&self) -> Self{
     Self::new(self.a.sin(),self.b*self.a.cos())
 }
 
 pub fn tan(&self) -> Self{
    Self::new(self.a.tan(),self.b/self.a.cos()*self.a.cos())
 }

 pub fn cosh(&self) -> Self{
    Self::new(self.a.cosh(),self.b*self.a.sinh())
 }
 
 pub fn sinh(&self) -> Self{
      Self::new(self.a.sinh(),self.b*self.a.cosh())
 }
 
 }
