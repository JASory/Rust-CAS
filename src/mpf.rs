/*
   Multiprecision Fixed point
*/
 use crate::traits::*;
 use number_theory::Mpz;
 use number_theory::NumberTheory;


 #[derive(Debug,Default,Clone, PartialEq)]
 pub struct Mpf{
    pub(crate) num: Mpz,
    point: usize,
 }
 
 impl Mpf{
 
 
 pub fn new(num: Mpz, point: usize) -> Self{
     Self{num,point}
 }
 
 
 pub fn from_string(x: &str) -> Option<Self>{
     let mut k = x.as_bytes().to_vec();
     
     while k[k.len()-1]==48{
       k.pop();
     }
    k.push(48);
     //println!("{:?} {}",k,k.len());
     let point = k.iter().position(|z| z == &46);
     let mut deci = 0usize;
     
     match point {
      Some(x) => deci = x,
      None => {} 
     }
     k.remove(deci);
     
     let p = unsafe {String::from_utf8_unchecked(k)};
      
      match Mpz::from_string(&p){
       Some(x) => return Some(Mpf::new(x,deci)),
       None => return None,
      }
     
     
     
 }
 
 pub fn from_mpz(num: Mpz) -> Self {
     Self{num, point: 0usize}
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
 
 //pub fn from_f64() -> Self{}
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
        Self::new(Mpz::one(),0usize)
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
     let scale = ((precision*2usize  + self.point)) as u64;
     let dec = Mpz::from_u64(5).pow(scale).shl((scale as usize));
     Self::new(self.num.ref_product(&dec).sqrt(),self.point+precision)
 }
     
 
 pub fn nth_root(&self, rt: u64, precision: usize) -> Self{
     let scale = (precision as usize*rt as usize) as u64+(self.point as u64 + rt+1);
     let dec = Mpz::from_u64(5).pow(scale).shl((scale as usize));
     Self::new(self.num.ref_product(&dec).nth_root(rt),self.point+precision)
 }
 
 pub fn pow(&self, p: u64) -> Self{
     let mut z = Mpf::default().mul_identity();
     let mut base = self.clone();
     let mut pow = p ; 
     if pow == 0 {
       return z
     }
     while pow > 1 {
       if pow&1 == 0{
       base = base.product(&base);
        pow>>=1;
       }
       else{
         z = z.product(&base);
         base = base.product(&base);
        pow = (pow-1)>>1; 
       }
     }
     return z.product(&base)
 }
 /*
 pub fn exp(&self, precision: usize) -> Self{
 
 }
 
 
 pub fn pow(&self, p: u64) -> Self{
    let 
 }
 
 pub fn ln(&self) -> Self{
    
 }
 */
 }
 
 
