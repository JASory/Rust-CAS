/*

  Quotient Ring Z[n]

*/

 fn euclid_gcd(p: i64 , q: i64)->(i64,i64,i64){
         let mut gcd : i64 =p; 
         let mut new_r : i64 =q;
         let mut bezout_1 : i64 =1;
         let mut new_s : i64 =0;
         let mut bezout_2 : i64 = 0;
         let mut new_t: i64 = 1;
    
    while new_r !=0 {
    let quotient =gcd/new_r;
    let mut temp : i64 =new_r;
    new_r=gcd-quotient*temp;
    gcd=temp;
    
    temp=new_s;
    new_s=bezout_1-quotient*temp;
    bezout_1=temp;
    
    temp=new_t;
    new_t=bezout_2-quotient*temp;
    bezout_2=temp;
    
    }
    (gcd,bezout_1,bezout_2)
}


 use crate::traits::*;
 use number_theory::NumberTheory;
 
 use crate::Mpz;

 
  #[derive(PartialEq,Clone,Debug, Default)]
  pub struct ZahlRing<T> where T: NumberTheory {
      n: T,
      a: T,
  }
  
  impl ZahlRing<u64> {
  
  pub fn unchecked_new(a: u64, n:u64) -> Self{
       Self{n,a}
  }
  
  pub fn new(a: u64, n: u64) -> Self{
       Self{a: a%n, n}
   }
   
  pub  fn gen_ring(n: u64) -> Self{
      Self{a: 0u64, n}
   }
   
  pub  fn set_element(&mut self, a: u64){
      self.a = a%self.n;
   }
  
  }
  
  impl Set for ZahlRing<u64> {
  
  fn rand(&self) -> Self{
      let mut k = self.clone();
      k.set_element(u64::default().rand());
      k
  }
  
  fn format(&self) -> String{
     self.a.to_string() + " mod " + &self.n.to_string()
  }
  
  }
  
  
  impl Magma for ZahlRing<u64> {
        fn op(&self, other: &Self)->Self {other.clone()}
 }
  
  impl AddIdentity for ZahlRing<u64>{
        
        fn add_identity(&self)->Self{
            Self::unchecked_new(0u64, self.n)
        }
  }
  
  impl AddInverse for ZahlRing<u64>{
        
        fn add_inverse(&self)->Self{
            Self::unchecked_new(self.n-self.a, self.n)
        }
  }
  
  impl AddOp for ZahlRing<u64>{
        
     fn addition(&self, other: &Self)->Self{
       if self.n != other.n {panic!("Unable to add elements of different rings")}
         Self::unchecked_new( (self.a.clone()%self.n.clone() + other.a.clone()%self.n.clone())%self.n.clone(), self.n.clone() )
     }
}

impl MulIdentity for ZahlRing<u64>{
     fn mul_identity(&self)->Self{
     Self::unchecked_new(1u64,self.n)
     }
  }
  
  impl MulOp for ZahlRing<u64>{
        
     fn product(&self, other: &Self) -> Self{
            if self.n != other.n {panic!("Unable to add elements of different rings")}
         Self::unchecked_new( (self.a%self.n * other.a%self.n)%self.n, self.n )
     }
}

 impl ZahlRing<u64>{
 
 pub fn mul_inverse(&self) -> Option<Self>{
     let modulo = self.n as i64;  
      let gcd = euclid_gcd(self.a as i64, modulo);
      if gcd.0 == 1 {
         Some( Self::unchecked_new( ( (gcd.1 % modulo + modulo)% modulo ) as u64, self.n ) )
      }
      else {
         None
      }
 } 
 
 }
 
  impl AdditiveGroup for ZahlRing<u64>{}
  impl SemiRing for ZahlRing<u64>{}
  impl Ring for ZahlRing<u64>{fn characteristic(&self)->u64 {self.n}}
  
  /*
      Mpz implementation
  */ 
  
  
  impl ZahlRing<Mpz> {
  
  pub fn unchecked_new(a: Mpz, n:Mpz) -> Self{
       Self{n,a}
  }
  
  pub fn new(a: Mpz, n: Mpz) -> Self{
       Self{a: a.euclidean_div(&n).1, n}
   }
   
  pub  fn gen_ring(n: Mpz) -> Self{
      Self{a: Mpz::zero(), n}
   }
   
  pub  fn set_element(&mut self, a: Mpz){
      self.a = a.euclidean_div(&self.n).1;
   }
  
  }
  
  impl Set for ZahlRing<Mpz> {
  
  fn rand(&self) -> Self{
      self.rand()
  }
  
  fn format(&self) -> String{
     self.a.to_string() + " mod " + &self.n.to_string()
  }
  
  }
  
  
  impl Magma for ZahlRing<Mpz> {
        fn op(&self, other: &Self)->Self {other.clone()}
 }
  
  impl AddIdentity for ZahlRing<Mpz>{
        
        fn add_identity(&self)->Self{
            Self::unchecked_new(Mpz::zero(), self.n.clone())
        }
  }
  
  impl AddInverse for ZahlRing<Mpz>{
        
        fn add_inverse(&self)->Self{
            Self::unchecked_new(self.n.ref_subtraction(&self.a), self.n.clone())
        }
  }
  
  impl AddOp for ZahlRing<Mpz>{
        
     fn addition(&self, other: &Self)->Self{
       if self.n != other.n {panic!("Unable to add elements of different rings")}
         Self::unchecked_new( self.a.ref_addition(&other.a).euclidean_div(&self.n).1, self.n.clone() )
     }
}

impl MulIdentity for ZahlRing<Mpz>{
     fn mul_identity(&self)->Self{
     Self::unchecked_new(Mpz::one(),self.n.clone())
     }
  }
  
  impl MulOp for ZahlRing<Mpz>{
        
     fn product(&self, other: &Self) -> Self{
            if self.n != other.n {panic!("Unable to add elements of different rings")}
         Self::unchecked_new( self.a.ref_product(&other.a).ref_euclidean(&self.n).1, self.n.clone() )
     }
}

 impl ZahlRing<Mpz>{
 
 pub fn mul_inverse(&self) -> Option<Self>{
     let modulo = self.n.clone();  
      let gcd = self.a.eea(&self.n);
      if gcd.0 == Mpz::one() {
         Some( Self::unchecked_new(gcd.1.ref_addition(&modulo).ref_euclidean(&modulo).1, self.n.clone() ) ) // W3kiUDjpppcdMcF
      }
      else {
         None
      }
 } 
 
 }
 
  impl AdditiveGroup for ZahlRing<Mpz>{}
  impl SemiRing for ZahlRing<Mpz>{}
  impl Ring for ZahlRing<Mpz>{fn characteristic(&self)-> u64 {0u64}}
  
  
