
pub fn euclid_gcd(p: i64 , q: i64)->(i64,i64,i64){
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





 use std::ops::*;
   use crate::traits::*;
   
   #[derive(Debug, Copy, Clone)]
 pub struct Quotient<const N: u64>{
               p: u64,
     }
     

impl<const N: u64> Quotient<N> {

   pub fn unchecked_new(p: u64)->Self{
          Quotient{p}
        }
    
  pub  fn new(p: u64)->Self{
           Self::unchecked_new(p%N)
       }
       
 }
 
  impl<const N: u64> Set for Quotient<N> {}
 
 impl<const N: u64> Magma for Quotient<N> {
        fn op(&self, other: Self)->Self {other}
 }
  
  impl<const N: u64> AddIdentity for Quotient<N>{
        
        fn add_identity()->Self{
            Self::unchecked_new(0u64)
        }
  }
  
  impl<const N: u64> AddInverse for Quotient<N>{
        
        fn add_inverse(&self)->Self{
            Self::unchecked_new(N-self.p)
        }
  }
  
  impl<const N: u64> Add for Quotient<N>{
        type Output= Self;
     fn add(self, other: Self)->Self{
         Self::unchecked_new( (self.p%N + other.p%N)%N )
     }
}

  impl<const N: u64> MulIdentity for Quotient<N>{
     fn mul_identity()->Self{
     Self::unchecked_new(1u64)
     }
  }

  impl<const N: u64> Mul for Quotient<N>{
        type Output= Self;
     fn mul(self, other: Self)->Self{
         Self::unchecked_new( (self.p%N * other.p%N)%N )
     }
}

  impl<const N: u64> AdditiveGroup for Quotient<N>{}
  impl<const N: u64> SemiRing for Quotient<N>{}
  impl<const N: u64> Ring for Quotient<N>{}

impl<const N: u64> Quotient<N> {
    
    fn sqr(&self)->Self{
        Self::unchecked_new( ((self.p as u128 * self.p as u128)% N as u128) as u64)
    }
    
    fn sqr_add(&self, x: u64)->Self{
        Self::unchecked_new( (((self.p as u128 * self.p as u128)% N as u128 + x as u128)%N as u128 ) as u64)
    }
    
    
    fn pow(&self, mut pow: u64)-> Self{
       let mut z = 1u128;
       let mut base = self.p as u128;
       let n = N as u128;
       if pow ==0 {
          return Self::unchecked_new(z as u64);
       }

      while pow > 1 {
  
        if pow%2 == 0 {
           base = base*base % n ;
           pow>>=1;
        }
  
     else{
  
       z = base*z % n;
    base = base*base % n;
     pow = (pow-1)>>1;  
   
     }
  }

 Self::unchecked_new( (base*z % n) as u64 )
    }
    
    
    
    fn mul_inverse(&self)->Option<Self>{
       let modulo = N as i64;  
      let gcd = euclid_gcd(self.p as i64, modulo);
      if gcd.0 == 1 {
         Some( Self::unchecked_new( ( (gcd.1 % modulo + modulo)% modulo ) as u64 ) )
      }
      else {
         None
      }
    }
    
}
