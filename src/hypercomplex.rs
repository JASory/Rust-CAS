
use std::convert::TryInto;

   use std::ops::*;
   use crate::traits::*;
   
   
  #[derive(Clone,Copy,Debug, PartialEq)]
  pub  struct HyperComplex<T, const S: usize>{
               re: [T;S],
              }
              
 impl<T: Clone, const S: usize> HyperComplex<T,S>{
 
 pub fn unchecked_new(re: [T;S])->Self{
        HyperComplex{re}
     }
     
 pub fn new(re: [T;S])->Option<Self>{
         if S.is_power_of_two(){
            return Some( Self::unchecked_new(re) ) ;
         }
           return None ;
       }
 pub fn unchecked_val(&self, x: usize)->T{
     self.re[x].clone()
 }
 
 pub fn to_vec(&self)->Vec<T>{
         self.re.iter().map(|x| x.clone()).collect::<Vec<T>>()
    }
    
 pub fn unchecked_from_vec(x: Vec<T>)->Self{
         let one  :  [T; S] = x.try_into().unwrap_or_else(|_| panic!("vector is wrong length"));
        HyperComplex::<T,S>::unchecked_new(one)
    }
 
 }
 
 impl<T, const S: usize> Set for HyperComplex<T,S>{}
  impl<T: Clone, const S: usize> Magma for HyperComplex<T,S>{
      fn op(&self, other: Self)->Self{other}
  }
 
 
 impl<T: AddIdentity + Clone, const S: usize> AddIdentity for HyperComplex<T,S>{
     
     fn add_identity()->Self{
          let mut k = vec![];
            for i in 0..S{
               k.push(T::add_identity())
             }
          Self::unchecked_from_vec(k)
     }
 }
 
 // AddInverse
 impl<T: AddInverse + Clone, const S: usize> AddInverse for HyperComplex<T,S>{
 
    fn add_inverse(&self)->Self{
          let mut k = self.clone();
            for i in k.re.iter_mut(){
               *i= i.add_inverse();
            }
        k
       }
 }
 
   // Real operations 
 impl<T: Ring + Clone, const S: usize> HyperComplex<T,S>{
     
   pub  fn real_add(&mut self, x: T){
        self.re[0]=self.re[0].clone() + x;
     }
     
    pub  fn real_sub(&mut self, x: T){
        self.re[0]= self.re[0].clone()+x.add_inverse();
     }
     
    pub  fn scalar(&mut self, x: T){
          for i in self.re.iter_mut(){
             *i= i.clone() * x.clone() ;
          }
      }
 } 
 

 impl<T: Add<Output =T> + Clone, const S: usize >Add for HyperComplex<T,S>{
             type Output = Self;
    fn add(self, other: Self)->Self{
       let mut k = self.clone();
       for (i,j) in k.re[..].iter_mut().zip(other.re[..].iter()){
            *i= i.clone() + j.clone();
        }
        k
    }
 
 }
 
  impl<T: MulIdentity + AddIdentity + Clone, const S: usize> MulIdentity for HyperComplex<T,S>{
 
     fn mul_identity()->Self{
        let mut k = Self::add_identity(); //HyperComplex{re: [T::mul_identity();S]};
        k.re[0]= T::mul_identity();
        k
     }
 
 }
 
 impl<T: AddIdentity + Add<Output =T> + Mul<Output = T> + Clone , const S: usize> HyperComplex<T,S>{
       
  pub  fn norm_sqr(&self)->T{
         let mut k = T::add_identity();
        for i in self.re[..].iter() {
            k= k + i.clone()*i.clone()
        }
        k
       }
 }
 
   impl<T: AddInverse + Clone, const S: usize> HyperComplex<T,S>{
 
 pub fn  conj(&self)->Self{
         let mut k = self.add_inverse();
           k.re[0]=k.re[0].add_inverse();
         k
      }
 }
/*
    Standard Complex
*/
 

 impl<T: Ring + Clone > Mul for HyperComplex<T,2> {
                type Output = Self;
 fn mul(self, other: Self)->Self{
      Self::unchecked_new(
        [self.re[0].clone()*other.re[0].clone() + (self.re[1].clone()*other.re[1].clone()).add_inverse(),
          self.re[0].clone()*other.re[1].clone() + self.re[1].clone()*other.re[0].clone()]
     )
 }
 
 }
 
 impl<T: Ring + MulInverse + Div<Output = T> + Clone> MulInverse for HyperComplex<T,2>{
 
    fn mul_inverse(&self)->Self{
       Self::unchecked_new(
       [self.re[0].clone() / self.norm_sqr() , (self.re[1].clone()/self.norm_sqr()).add_inverse() ]
       )
    }
 }
 
 
 impl HyperComplex<f64,2>{
 
 pub  fn arg(&self)->f64{
      self.re[1].atan2(self.re[0])
   }
   
 pub  fn norm(&self)->f64{
       self.norm_sqr().sqrt()
   }
      // ratio of self / norm
 pub  fn normalize(&self)->Self{
       Self::unchecked_new([self.re[0]/self.norm(),self.re[1]/self.norm()])
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
 
 impl<T: AdditiveGroup + Clone> AdditiveGroup for HyperComplex<T,2> {} 
 impl<T: AdditiveGroup + MultiplicativeGroup + Div<Output =T> + Ring + Clone> MultiplicativeGroup for HyperComplex<T,2>{}
 impl<T: AdditiveGroup + MultiplicativeGroup + Div<Output =T> + Ring   + Clone> SemiRing for HyperComplex<T,2>{}
 impl<T: AdditiveGroup + MultiplicativeGroup + Div<Output =T> + Ring  + Clone> Ring for  HyperComplex<T,2>{}
 impl<T: Field + AdditiveGroup + MultiplicativeGroup + Div<Output =T> + Ring + Clone> Field for HyperComplex<T,2>{}
 
 
 impl   HyperComplex<u64,2>{
 
  pub  fn irreducible(self)->bool{
         if self.re[0] == u64::add_identity() && self.re[1] != u64::add_identity(){
           if self.re[1].form(4,3)&self.re[1].irreducible(){
               return true
           }
           else {
              return false
           }
         }
         else if self.re[1] == u64::add_identity() && self.re[0] != u64::add_identity(){
              if self.re[0].form(4,3)&self.re[0].irreducible(){
               return true
           }
           else {
              return false
           }
         }
         else if  self.norm_sqr().form(4,3)== false && self.norm_sqr().irreducible(){
          return true
         }
         else {
            return false
         }
     }
 
 }
 
 
 /*
 
  z =  a,b  y = c,d
 
 a*c -d.conj()*b , d*a + b*c.conj()
 */
 
 
 
 
 //Quaternion operations
 impl<T:  Mul<Output=T> + Add<Output =T> + Sub<Output =T> + Clone > Mul for HyperComplex<T,4> {
         type Output = Self;
  fn  mul(self, other: Self)->Self{
      Self::unchecked_new([
      self.re[0].clone()*other.re[0].clone()  - self.re[1].clone()*other.re[1].clone() - 
      self.re[2].clone()*other.re[2].clone() - self.re[3].clone()*other.re[3].clone(),
         
      self.re[0].clone()* other.re[1].clone() + self.re[1].clone()*other.re[0].clone() + 
      self.re[2].clone()*other.re[3].clone() - self.re[3].clone()*other.re[2].clone(),
         
      self.re[0].clone()* other.re[2].clone() - self.re[1].clone()*other.re[3].clone() + 
      self.re[2].clone()*other.re[0].clone() + self.re[3].clone()*other.re[1].clone(),
         
      self.re[0].clone()* other.re[3].clone() + self.re[1].clone()*other.re[2].clone() - 
      self.re[2].clone()*other.re[1].clone() + self.re[3].clone()*other.re[0].clone()
      ])
  }
   
 }
 
 impl HyperComplex<f64,4>{
 
  pub  fn norm(&self)->f64{
          self.norm_sqr().sqrt()
  }
 
  pub  fn normalize(&self)->Self{
          Self::unchecked_new([self.re[0]/self.norm(),self.re[1]/self.norm(),self.re[2]/self.norm(),self.re[3]/self.norm()] )
  }
  /*
  pub mul_inverse(&self)-> Option<Self>{
       
  }
 */
 }
 
  //Lipschitz
 impl  HyperComplex<u64,4>{
     
      fn irreducible(self)->bool{
         self.norm_sqr().irreducible()
     } 
 }
  
  
    #[derive(Clone, Copy,Debug)]
  pub  struct Dual<T>{
           a: T,
           b: T,
  }
  
  impl<T> Dual<T>{
  
  pub fn unchecked_new(a: T, b: T)->Self{
         Self{a,b}
      }
  }
  
  impl<T: Add<Output = T> > Add for Dual<T>{
      type Output = Self;
    fn add(self, other: Self)->Self{
       Self::unchecked_new(self.a + other.a, self.b+other.b)
    }  
  }
  
  impl<T: Add<Output = T> + Mul<Output = T> + Clone> Mul for Dual<T>{
         type Output = Self;
    fn mul(self, other: Self)->Self{
       Self::unchecked_new(self.a.clone()*other.a.clone(), self.a*other.b + self.b*other.a)
    }     
  }
              
