
use std::convert::TryInto;

   use std::ops::*;
   use crate::traits::*;
   use crate::blas1::*;
   
   
  #[derive(Clone,Copy,Debug, PartialEq)]
  pub  struct HyperComplex<T, const S: usize> where T: Clone + Set{
               re: [T;S],
              }
              
 impl<T: Set + Clone, const S: usize> HyperComplex<T,S>{
 
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
   
  pub fn unchecked_from_slice(x: &[T])->Self{  
          let mut k = vec![];
          k.extend_from_slice(x);
          Self::unchecked_from_vec(k)
  }
 
 }
 
 impl<T: Set + Clone, const S: usize> Set for HyperComplex<T,S>{
      fn rand()->Self{
          let mut k = vec![];
            for i in 0..S{
               k.push(T::rand())
             }
          Self::unchecked_from_vec(k)
      }
      
      fn format(&self)->String{
          self.re.iter().map(|x| x.format()).collect::<Vec<String>>().join(",")
      }
 }
 
  impl<T: Set + Clone, const S: usize> Magma for HyperComplex<T,S>{
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
            add_inv(&mut k.re[..]);
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
 

 impl<T: Add<Output =T> + Clone + Set, const S: usize >Add for HyperComplex<T,S>{
             type Output = Self;
    fn add(self, other: Self)->Self{
       let mut k = self.clone();
        add(&other.re[..],&mut k.re[..]);
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
         let mut k = self.clone();
          add_inv(&mut k.re[1..]);
         k
      }
 }
 /*
     Operations for f64 Hypercomplex extend to all Fields if possible
 */
 
 
 impl<const S: usize> HyperComplex<f64,S>{
 
 pub fn norm(&self)->f64{
         self.norm_sqr().sqrt()
 }
 
 pub fn normalize(&mut self){
          let norm = self.norm();
         for i in self.re.iter_mut(){
            *i /=norm
         }
 }
 
 }
/*
    Standard Complex
*/
 

 impl<T: Ring + Clone > Mul for HyperComplex<T,2> {
                type Output = Self;
 fn mul(self, other: Self)->Self{
    Self::unchecked_new(
    [ self.re[0].clone()*other.re[0].clone() + (self.re[1].clone()*other.re[1].clone()).add_inverse(),
      self.re[0].clone()*other.re[1].clone() + self.re[1].clone()*other.re[0].clone()]
    )
 }
 
 }
 
 
 
 impl<T:  Field + Clone> MulInverse for HyperComplex<T,2>{
 
    fn mul_inverse(&self)->Self{
        let norm_inv = self.norm_sqr().mul_inverse();
       Self::unchecked_new(
       [self.re[0].clone() * norm_inv.clone() , self.re[1].clone().add_inverse()* norm_inv ]
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
 
 impl<T:  AdditiveGroup + Clone> AdditiveGroup for HyperComplex<T,2> {} 
 impl<T:  Field + Clone> MultiplicativeGroup for HyperComplex<T,2>{}
 impl<T:  Field + Clone> SemiRing for HyperComplex<T,2>{}
 impl<T:  Field + Clone> Ring for  HyperComplex<T,2>{fn characteristic()->u64 {0u64}}
 impl<T:  Field + Clone> Field for HyperComplex<T,2>{}
 
 
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
 /*    
 pub  fn factor(self)->Vec<Self>{
      
         
    
 }    
 */
 }
 

 
 /*
 
  z =  a,b  y = c,d
 
 a*c -d.conj()*b , d*a + b*c.conj()
 */
 
 
 /*
    Quaternions 
 
 */
 
 
 
 //Quaternion operations
 impl<T:  Ring + Clone > Mul for HyperComplex<T,4> {
         type Output = Self;
  fn  mul(self, other: Self)->Self{
      Self::unchecked_new([
      self.re[0].clone()*other.re[0].clone() + self.re[1].clone()*other.re[1].clone().add_inverse() + 
      self.re[2].clone()*other.re[2].clone().add_inverse() + self.re[3].clone()*other.re[3].clone().add_inverse(),
      
      self.re[0].clone()*other.re[1].clone() + self.re[1].clone()*other.re[0].clone() + 
      self.re[2].clone()*other.re[3].clone() + self.re[3].clone()*other.re[2].clone().add_inverse(),
      
      self.re[0].clone()*other.re[2].clone() + self.re[1].clone()*other.re[3].clone().add_inverse() + 
      self.re[2].clone()*other.re[0].clone() + self.re[3].clone()*other.re[1].clone(),
      
      self.re[0].clone()*other.re[3].clone() + self.re[1].clone()*other.re[2].clone() + 
      self.re[2].clone()*other.re[1].clone().add_inverse() + self.re[3].clone()*other.re[0].clone()
      ])
  }
   
 }
 
 
 impl HyperComplex<f64,4>{
 
  pub  fn mul_inverse(&self)-> Self{
       let n = self.norm_sqr();
       Self::unchecked_new([
       self.re[0]/n, self.re[1].add_inverse()/n,self.re[2].add_inverse()/n, self.re[3].add_inverse()/n
       ])
  }
 
 }
 
  //Lipschitz
 impl  HyperComplex<u64,4>{
     
      fn irreducible(self)->bool{
         self.norm_sqr().irreducible()
     } 
 }
 
 /*  Octonions */
 
 impl<T:  Ring + Clone + Copy  + std::fmt::Debug> Mul for HyperComplex<T,8> {
         type Output = Self;
  fn  mul(self, other: Self)->Self{
        let (a,b) = ( HyperComplex::<T,4>::unchecked_new([self.re[0], self.re[1], self.re[2], self.re[3]]), 
                      HyperComplex::<T,4>::unchecked_new([self.re[4], self.re[5], self.re[6], self.re[7]]) );
        let (c,d) = ( HyperComplex::<T,4>::unchecked_new([other.re[0], other.re[1], other.re[2], other.re[3]]), 
                      HyperComplex::<T,4>::unchecked_new([other.re[4], other.re[5], other.re[6], other.re[7]]) );
                      
        let part1 = (a.clone() * c.clone()) +   (d.conj().clone() * b.clone() ).add_inverse() ;
               // println!("{:?}",&part1.re[..]);
        let part2 = ( d * a ) +   (b * c.conj())      ;
        HyperComplex::<T,8>::unchecked_new([part1.re[0],part1.re[1],part1.re[2],part1.re[3], part2.re[0], part2.re[1], part2.re[2], part2.re[3]])
 }
 }
 
 impl HyperComplex<f64,8>{
 
  
  
  pub fn mul_inverse(&self)->Self{
                 let n = self.norm_sqr();
         Self::unchecked_new([
       self.re[0]/n, self.re[1].add_inverse()/n,self.re[2].add_inverse()/n, self.re[3].add_inverse()/n,
       self.re[4]/n, self.re[5].add_inverse()/n,self.re[6].add_inverse()/n, self.re[7].add_inverse()/n,
       ])
  }  
      
 }
 
  /*  Gaussian, Eisenstein, Kleinian  (-1,-3,-7)
   pub struct QuadraticInteger<const D: i64>{
               re : [i64;2]
   }
       
       impl QuadraticInteger<const D: i64>{
            
            fn unchecked_new(re: [i64;2])->Self{
                Self{re}
            }
            
           // fn new
            
            fn add_identity()->Self{
                Self::unchecked_new([0i64, 0i64])
            }
            
            fn add_inverse(&self){
                 Self::unchecked_new([-self.re[0], -self.re[1]])
            }
            
            fn add(&self, other: Self)->Self{
                Self::unchecked_new(self.re[0]+other.re[0], self.re[1]+other.re[1])
            }
            
            fn mul(&self, other: Self)->Self{
                if D < 0i64 {
                   if 4i64-(D%4i64) == 1{// Check if -D mod 4 == 1
                      return  Self::unchecked_new([
                      
                      ])
                   }
                   else {// 
                     return Self::unchecked_new([
                       self.re[0]*other.re[0] + self.re[1]*other.re[1]*D,
                        self.re[0]*other.re[1] + self.re[1]*other.re[0]
                        ])
                   }
                }
                else{
                   if (D%4i64) == 1{// Check if D mod 4 == 1
                      
                   }
                   else {
                   return Self::unchecked_new([
                        self.re[0]*other.re[0] + self.re[1]*other.re[1]*D,
                        self.re[0]*other.re[1] + self.re[1]*other.re[0]
                        ])
                   }
                } 
            }
            
            fn norm(&self)->i64{
                 ((self.re[0]*self.re[0] - D*self.re[1]*self.re[1]) as f64).sqrt() as i64
            }
            
            fn conj(&self)->Self{
                Self::unchecked_new(self.re[0], -self.re[1])
            }
       }
  */
  
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
  
  impl<T: Add<Output = T> + Mul<Output = T> 
+ Clone> Mul for Dual<T>{
         type Output = Self;
    fn mul(self, other: Self)->Self{
       Self::unchecked_new(self.a.clone()*other.a.clone(), self.a*other.b + self.b*other.a)
    }     
  }
              
