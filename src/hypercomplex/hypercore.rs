/*
  Core HyperComplex functions 
*/

 use crate::traits::*;
 use crate::blas::lvlone::*;

 #[derive(Clone,Debug, PartialEq)]
  pub  struct HyperComplex<T, const S: usize> where T: Set{
         pub(crate)   re: [T;S],
              }
              
 
 impl<T: Set , const S: usize> HyperComplex<T,S>{
 
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
 
 impl<T:Set , const S: usize> Default  for HyperComplex<T,S>{
 
  fn default() -> Self{
  
   let x = (0..S).map(|_| T::default().clone() ).collect::<Vec<T>>();
    let one  :  [T; S] = x.try_into().unwrap_or_else(|_| panic!("vector is wrong length"));
        HyperComplex::<T,S>::unchecked_new(one)
    
    }
 }           
 
 impl<T: Set , const S: usize> Set for HyperComplex<T,S>{
      fn rand(&self)->Self{
          let mut k = vec![];
            for i in 0..S{
               k.push(T::default().rand())
             }
          Self::unchecked_from_vec(k)
      }
      
      fn format(&self)->String{
          self.re.iter().map(|x| x.format()).collect::<Vec<String>>().join(" + ")
      }
 }
 
  impl<T: Set , const S: usize> Magma for HyperComplex<T,S>{
      fn op(&self, other: &Self)->Self{other.clone()}
  }
 
 
 impl<T: AddIdentity, const S: usize> AddIdentity for HyperComplex<T,S>{
     
     fn add_identity(&self)->Self{
          let mut k = vec![];
            for i in 0..S{
               k.push(T::default().add_identity())
             }
          Self::unchecked_from_vec(k)
     }
 }
 
 // AddInverse
 impl<T: AddInverse , const S: usize> AddInverse for HyperComplex<T,S>{
 
    fn add_inverse(&self)->Self{
          let mut k = self.clone();
            add_inv(&mut k.re[..]);
        k
       }
 }
 
   // Real operations 
 impl<T: Ring , const S: usize> HyperComplex<T,S>{
     
   pub  fn real_add(&mut self, x: T){
        self.re[0]=self.re[0].addition(&x);
     }
     
    pub  fn real_sub(&mut self, x: T){
        self.re[0]= self.re[0].addition(&x.add_inverse());
     }
     
    pub  fn scalar(&mut self, x: T){
          for i in self.re.iter_mut(){
             *i= i.product(&x) ;
          }
      }
 } 
 

 impl<T: AddOp + Set, const S: usize >AddOp for HyperComplex<T,S>{

    fn addition(&self, other: &Self)->Self{
       let mut k = self.clone();
        add(&other.re[..],&mut k.re[..]);
        k
    }
 
 }
 
  impl<T: MulIdentity + AddIdentity, const S: usize> MulIdentity for HyperComplex<T,S>{
 
     fn mul_identity(&self)->Self{
        let mut k = Self::default().add_identity(); //HyperComplex{re: [T::mul_identity();S]};
        k.re[0]= T::default().mul_identity();
        k
     }
 
 }
 
 impl<T: AddIdentity + AddOp + MulOp, const S: usize> HyperComplex<T,S>{
       
  pub  fn norm_sqr(&self)->T{
         let mut k = T::default().add_identity();
        for i in self.re[..].iter() {
            k= k.addition(&i.product(&i))
        }
        k
       }
 }
 
   impl<T: AddInverse , const S: usize> HyperComplex<T,S>{
 
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
 
 pub fn normalize(&mut self) -> f64{
          let norm = self.norm();
         for i in self.re.iter_mut(){
            *i /=norm
         }
         norm
 }
 
 }      
