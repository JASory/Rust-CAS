

   use std::ops::*;
   use crate::traits::*;
   
  #[derive(Clone, Copy,Debug)]
  pub  struct HyperComplex<T, const S: usize>{
               re: [T;S],
              }
              
 impl<T, const S: usize> HyperComplex<T,S>{
 
 pub fn unchecked_new(re: [T;S])->Self{
        HyperComplex{re}
     }
     
 pub fn new(re: [T;S])->Option<Self>{
         if S.is_power_of_two(){// re.len()%2==0{ S.is_power_of_two()
            return Some( Self::unchecked_new(re) ) ;
         }
           return None ;
       }
 
 }
 
 impl<T: AddIdentity + Clone + Copy, const S: usize> AddIdentity for HyperComplex<T,S>{
 
     fn add_identity()->Self{
        HyperComplex{re: [T::add_identity();S]}
     }
 
 }
 
 impl<T: AddInverse + Clone, const S: usize> AddInverse for HyperComplex<T,S>{
 
    fn add_inverse(&self)->Self{
       let mut k = self.clone();
       for i in k.re.iter_mut(){
       *i= i.add_inverse();
       }
       k
    }
 }
 
 
 impl<T: AddAssign + Clone, const S: usize> AddAssign for HyperComplex<T,S>{
      
       fn add_assign(&mut self, other: Self){
        for (i,j) in self.re[..].iter_mut().zip(other.re[..].iter()){
            *i+=j.clone();
        }
    }
 }
 
  impl<T: MulIdentity + Clone + Copy, const S: usize> MulIdentity for HyperComplex<T,S>{
 
     fn mul_identity()->Self{
        HyperComplex{re: [T::mul_identity();S]}
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

 
 // standard complex operations
 impl<T: MulOperation + Mul<Output=T> + Add<Output =T> + Sub<Output =T> + Clone > MulOperation for HyperComplex<T,2> {
 
 fn multiply(self, other: Self)->Self{
    Self::unchecked_new(
    [self.re[0].clone()*other.re[0].clone() - self.re[1].clone()*other.re[1].clone(),
      self.re[0].clone()*other.re[1].clone() + self.re[1].clone()*other.re[0].clone()]
    )
 }
 
 }
 
 impl<T: MulInverse + AddInverse + AddIdentity + Mul<Output =T> + Add<Output = T> + Div<Output = T> + Clone> MulInverse for HyperComplex<T,2>{
 
    fn mul_inverse(&self)->Self{
       Self::unchecked_new(
       [self.re[0].clone() / self.norm_sqr() , (self.re[1].clone()/self.norm_sqr()).add_inverse() ]
       )
    }
 }
 
 
 impl HyperComplex<f64,2>{
 
 pub  fn polar(&self)->(f64,f64){
       ( self.norm_sqr().sqrt() ,3.14159265359 -(self.re[1]/self.re[0]).atan() )
   }
   
 pub  fn from_polar(r: f64, theta: f64)->Self{
      Self::unchecked_new( [r*theta.cos(), r*theta.sin()])
   }
   
  pub fn nthroot(&self, n: u64)->Vec<Self>{ // returns a nth root
      let (r,x) = self.polar();
      let mut nthroots = vec![];
      let radius = r.powf( (n as f64).recip() );
      for i in 0..n {
      nthroots.push( 
      Self::unchecked_new( [radius* ((x + 6.2831853071795*i as f64)/n as f64).cos() , radius* ((x + 6.2831853071795*i as f64)/n as f64).sin()] )
      ) ;
      }
      nthroots
   }
 }  
 
 impl HyperComplex<u64,2>{
 
  pub fn irreducible(self)->bool{
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
 impl<T: MulOperation + Mul<Output=T> + Add<Output =T> + Sub<Output =T> + Clone > MulOperation for HyperComplex<T,4> {
 
  fn  multiply(self, other: Self)->Self{
      Self::unchecked_new([
      self.re[0].clone()*other.re[0].clone()  - self.re[1].clone()*other.re[1].clone() - self.re[2].clone()*other.re[2].clone() - self.re[3].clone()*other.re[3].clone(),
      self.re[0].clone()* other.re[1].clone() + self.re[1].clone()*other.re[0].clone() + self.re[2].clone()*other.re[3].clone() - self.re[3].clone()*other.re[2].clone(),
      self.re[0].clone()* other.re[2].clone() - self.re[1].clone()*other.re[3].clone() + self.re[2].clone()*other.re[0].clone() + self.re[3].clone()*other.re[1].clone(),
      self.re[0].clone()* other.re[3].clone() + self.re[1].clone()*other.re[2].clone() - self.re[2].clone()*other.re[1].clone() + self.re[3].clone()*other.re[0].clone()
      ])
  }
   
 }
  //Lipschitz
 impl HyperComplex<u64,4>{
     
  pub fn irreducible(&self)->bool{
         self.norm_sqr().irreducible()
     } 
 }
  
              
