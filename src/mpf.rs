/*
   Multiprecision Fixed point
*/
 use crate::traits::*;
 use number_theory::Mpz;
 use number_theory::NumberTheory;

 /*
 pub fn formatter(x: String, splice: usize) -> String{
     
 } 
*/
 #[derive(Debug,Default,Clone, PartialEq)]
 pub struct Mpf{
    num: Mpz,
    point: usize,
 }
 
 impl Mpf{
 
 
 pub fn new(num: Mpz, point: usize) -> Self{
     Self{num,point}
 }
 
 pub fn from_string(x: &str) -> Self{
       
        let deci = x.chars().position(|x| x == '.').unwrap();
        let mut s = x.to_owned();
        
        s.remove(deci);
        //println!("{}",s);
        Self::new(Mpz::from_string(&s).unwrap(),x.len()-1-deci)

 }
      // Fix for negative values smaller than 1
 pub fn to_string(&self) -> String{
        let mut p = self.num.to_string();
        if self.point >= p.len(){
           let zeros = (0..(self.point-p.len())+1).map(|_|'0').collect::<String>();
           p = zeros + &p;
        }
        p.insert(p.len()-self.point,'.');
        p
 }
 
 
 }
 
 impl Set for Mpf{
 
  fn rand(&self) -> Self{
     Self::new(self.num.rand(),self.point)
  }
  
  fn format(&self) -> String{
      self.to_string()
  }
 
 }
 
 impl Magma for Mpf{
 
  fn op(&self, other: &Self) -> Self{
      other.clone()
  }
 
 }
 
 impl AddIdentity for Mpf{
 
    fn add_identity(&self) -> Self{
        Self::new(Mpz::zero(), 1usize)
    }
 }

 impl AddOp for Mpf{
    
    fn addition(&self, other: &Self) -> Self{
       let len =  self.point.abs_diff(other.point);
       if len == 0{
          return Self::new(self.num.addition(&other.num),self.point)
       }
       let scalar = Mpz::from_u64(10).pow(len as u64);
       if self.point > other.point {
         let k = other.num.product(&scalar);
        return Self::new(k.addition(&self.num),self.point)
       }
       else {
         let k = self.num.product(&scalar);
         return Self::new(k.addition(&other.num),other.point)
       }
    }
 }
 
 impl AddInverse for Mpf{
    
    fn add_inverse(&self) -> Self{
       Self::new(self.num.add_inverse(), self.point)
    }
 }
 
 impl MulIdentity for Mpf{
 
    fn mul_identity(&self) -> Self{
        Self::new(Mpz::one(),1usize)
    }
 }
 
 impl MulOp for Mpf{
    
    fn product(&self, other: &Self) -> Self{
       Self::new(self.num.ref_product(&other.num),self.point+other.point)
    }
 }
 /*
 impl MulInverse for Mpf{
    
    fn mul_inverse(&self) -> Self{
       
    }
 }
 */
 
 impl Mpf{
 
 pub fn division(&self, other: &Self, precision: usize) -> Self{
          let scale = (precision- (self.point-other.point)) as u64;
          let dec = Mpz::from_u64(5).pow(scale).shl((scale as usize));
          Self::new(self.num.ref_product(&dec).euclidean_div(&other.num).0,self.point+scale as usize-other.point)
 }
 
 pub fn sqrt(&self, precision: usize) -> Self{
     let scale = (precision*2  + self.point) as u64;
     let dec = Mpz::from_u64(5).pow(scale).shl((scale as usize));
     Self::new(self.num.ref_product(&dec).sqrt(),self.point+precision)
 }
 /*
 pub fn pow(&self, p: u64) -> Self{
    let 
 }
 
 pub fn ln(&self) -> Self{
    
 }
 */
 }
 
 
