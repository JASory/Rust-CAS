
 use crate::traits::*;
 use std::ops::*;
 

#[derive(Clone,Copy,Debug, PartialEq)]
pub struct Galois<const P : u64, const S : usize>{
       coef: [u64;S],
}

impl<const P : u64, const S : usize> Galois<P,S>{

 pub fn unchecked_new(coef: [u64;S])->Self{
        Self{coef}
 }
 
 pub fn new(coef: [u64;S])->Option<Self>{
    if P.irreducible(){
      return Some(Self::unchecked_new(coef))
    }
    else{
      return None
    }
 }

}

impl <const P: u64, const S: usize> Set for Galois<P,S>{

  fn rand()->Self{
      let mut k = Self::add_identity();
      for i in k.coef.iter_mut(){
       *i = u64::rand()%P
      }
      k
  }
  
  fn format(&self)->String{
      let limit = self.coef.len()-1;
      let  mut p = Vec::new();
      for i in 0..self.coef.len(){
        p.push(self.coef[limit-i].format() + "x^" + &(limit -i).to_string() )
      }
      p.join(" + ")
  }

}

impl <const P: u64, const S: usize> Magma for Galois<P,S>{
     fn op(&self, other: Self)->Self{
         other
     }
}

impl <const P: u64, const S: usize> AddIdentity for Galois<P,S>{
   fn add_identity()->Self{
      Self::unchecked_new([0u64;S])
   }
}

impl<const P: u64, const S: usize >Add for Galois<P,S>{
             type Output = Self;
    fn add(self, other: Self)->Self{
       let mut k = self.clone();
       for (i,j) in k.coef[..].iter_mut().zip(other.coef[..].iter()){
            *i= (*i + j)%P;
        }
        k
    }
 
 }
 
 impl<const P: u64, const S: usize>AddInverse for Galois<P,S>{
 
   fn add_inverse(&self)->Self{
      let mut k = self.clone();
       for i in k.coef[..].iter_mut(){
            *i=  P-*i;
        }
        k
    }
   }
   
  impl<const P : u64, const S: usize>MulIdentity for Galois<P,S>{
  
  fn mul_identity()->Self{
     let mut k = Self::add_identity();
     k.coef[0]=1u64;
     k
  }
  }
  
 //   impl<const P : u64, const S: usize>Mul for Galois<P,S>{
  
  
