/*
 Precursor functions

*/

   use std::ops::*;
   
use crate::numeric::Sign;
use crate::traits::*;
use crate::numbertheorectic::*;





 // Multiply accumulate 
 #[inline]
pub fn mac(carry: u64, x: u64, y: u64, output: &mut u64 )->u64{
    let interim = carry as u128 + x as u128 * y as u128 + *output as u128;
    *output = interim as u64;
    (interim>>64u64 ) as u64
}

   // In-place Multiply-add carry
fn carry_mul(carry: u64, x: u64, y: &u64, output: &mut u64)->u64{

    let product = (x as u128 * *y as u128) + carry as u128 ;

        *output=product as u64;
          (product>>64) as u64
   }
   
 fn carry_div(carry: u64, x: u64, y: u64, output: &mut u64)->u64{
    let num = unsafe { std::mem::transmute::<(u64,u64), u128>((x,carry)) };
    let factor = num/y as u128;
     *output = factor as u64; 
    (num %y as u128) as u64
}


   // In-place shift-right, returns the extra bits that were shifted out
pub fn carry_shr(carry: u64, x: u64, places: u32, output: &mut u64)->u64{
    *output = (x>>places)|carry ;
    if places == 0{
        return 0
    }
    unsafe  { core::arch::x86_64::_bextr_u64(x,0,places)<<(64-places)/*.overflowing_shl(65-places).0<<(64-places)*/ }
}

   // In-place shift-left, returns the extra bits that were shifted out
pub fn carry_shl(carry: u64, x: u64, places: u32, output: &mut u64)->u64{
   *output = (x.overflowing_shl(places).0)|carry ;
       unsafe  { core::arch::x86_64::_bextr_u64(x,64-places,64) }
}



/*
   Bitwise slice operations 
*/

  //bitwise AND starting at 
 fn and(x : &mut [u64], y: &[u64]){
    for (i,j) in x.iter_mut().zip(y.iter()){                             
        *i&=j
       }
 }
     //Bitwise OR
 fn or(x: &mut [u64], y: &[u64]){
    for (i,j) in x.iter_mut().zip(y.iter()){                             
        *i|=j
       }
 }
     //Bitwise XOR
 fn xor(x: &mut [u64], y: &[u64]){
    for (i,j) in x.iter_mut().zip(y.iter()){                             
        *i^=j
       }
 }
   // slice shift-right
fn slice_shr(x: &mut[u64],shift: u32)-> u64{

  let mut carry= 0u64;
  
  for i in x.iter_mut().rev(){
  
   carry = carry_shr(carry, *i, shift,i);
  
  }
  
  carry

}  
  // slice shift-left
fn slice_shl(x: &mut[u64], shift: u32)-> u64{

   let mut carry = 0u64;

     for i in x.iter_mut(){
         
           carry = carry_shl(carry, *i, shift,i);
           
      }
      
   carry
}

/*
 Arithmetic slice operations 
*/

 fn unequal_add(x: &mut [u64],y: &[u64])->u8{  //first number must be larger

    let mut carry = 0u8;
 
    let (lo,hi) = x.split_at_mut(y.len()); //split to make equal
    
    for (i,j) in lo.iter_mut().zip(y.iter()){                               //add equal 
            carry = unsafe{core::arch::x86_64::_addcarry_u64(carry,*i,*j,i)}
        }
      
      if carry > 0u8{
       
       for k in hi.iter_mut(){   //add the carry until there is no carry
       carry = unsafe{core::arch::x86_64::_addcarry_u64(carry,*k,0u64,k)};
       if carry == 0u8{
       break;
       }
       }
      
      }
   carry
 }
 
 fn offset_add(x: &mut[u64],y: &mut[u64], offset: usize)->u8{  //adds the second value to the first, shifted by offset

    let (lo,hi) = x.split_at_mut(offset);
 //uint/shift.rs
     unequal_add(hi,y)
 }
 /*
    Subtraction x-y
 */
 
 fn unequal_sub(x: &mut [u64],y: &[u64])->u8{  //first number must be larger

 let mut carry = 0u8;
 
 let (lo,hi) = x.split_at_mut(y.len()); //split to make equal
    
 for (i,j) in lo.iter_mut().zip(y.iter()){                               //add equal 
        carry = unsafe{core::arch::x86_64::_subborrow_u64(carry,*i,*j,i)}
       }
      
      if carry > 0u8{
       
       for k in hi.iter_mut(){   //add the carry until there is no carry
       carry = unsafe{core::arch::x86_64::_subborrow_u64(carry,*k,0u64,k)};
       if carry == 0u8{
       break;
       }
       }
      
      }
   carry

}


 fn  scalar_slice(x: &mut [u64], scale : u64)->u64{
      
   let mut carry = 0u64;
   
   for i in x.iter_mut() {
     carry = carry_mul(carry,*i,&scale, i);
   }   
   carry
  }

fn  scalar_slice2(x: &[u64], scale : u64, output: &mut [u64]){
      
   let mut carry = 0u64;
   
   for (i,j) in x.iter().zip(output.iter_mut()) {
     carry = carry_mul(carry,*i,&scale, j);
   }
   if carry > 0 {
       output[x.len()]=carry;
   }
  }
  
 pub fn mul_slice2(x: &[u64], y: &[u64], output : &mut [u64]){
      let mut scalevec = vec![0u64;x.len()+1];
      for i in 0..y.len(){
          scalar_slice2(&x[..],y[i],&mut scalevec[..]);
          unequal_add(&mut output[i..], &scalevec[..]);
      }
  }


pub fn mul_slice(x: &[u64], y: &[u64], output: &mut[u64])->u64{
    let mut carry = 0u64;
    for i in 0..y.len(){
      for j in 0..x.len()  {
          carry = mac(carry,x[j],y[i],  &mut output[j +i]);
      }
      output[i + y.len()]  = carry;
      carry = 0;
    }
    carry
}
  
   // divides inplace and returns remainder
 fn div_slice(x : &mut[u64], divisor: u64 )->u64{
 
 let mut carry = 0u64;
   
   for i in x.iter_mut().rev() {
     carry = carry_div(carry,*i,divisor, i);
   }   
   carry
 }



  

 /*
 Mpz struct
 
 */
 
 #[derive(Debug,Clone, PartialEq)]
 pub struct Mpz{
         sign: Sign,
     elements: Vec<u64>,
 }
 
 impl Set for Mpz{
     fn rand()->Mpz{
        Self::new(Sign::Positive, (0..100).map(|_| u64::rand()).collect::<Vec<u64>>())
     }
     
     fn format(&self)->String{
         self.elements.iter().rev().map(|x| x.format()).collect::<Vec<String>>().join(",")
     }
 }
 impl Magma for Mpz{
       fn op(&self, other: Self)->Self{other}
 }
 
 
 impl Mpz{
  pub fn new(sign: Sign, elements: Vec<u64>)->Self{// New 
         Mpz{sign, elements}
     }
     
  pub   fn neg(mut self){// negation
        self.sign = self.sign.neg();
     }
     
  pub  fn is_even(&self)->bool{// checks if even 
           self.elements[0]&1==0
       }
     
  pub   fn len(&self)->usize{
           self.elements.len()
        }
     
  pub   fn leading_zeros(&self)-> u64{// leading zeros
     
           let mut idx : u64 =0;
    
            for i in self.elements.iter().rev(){
                if i == &0u64{
                   idx+=1;
                }
                else{
                  break;
                }
            }
            if idx == self.len() as u64{
               return 64u64*idx
            }
            else{
              return self.elements[self.len()-1usize-idx as usize].leading_zeros() as u64 + 64u64*idx
            }
        }
        
        
 pub   fn trailing_zeros(&self)-> u64{// Trailing zeros
          let mut idx : u64 =0;
    
            for i in self.elements.iter(){
                if i == &0u64{
                   idx+=1;
                }
                else{
                  break;
                }
            }
        if idx == self.len() as u64{
               return 64u64*idx
            }
            else{
              return self.elements[idx as usize].leading_zeros() as u64 + 64u64*idx
            }
       }       
       
        
 pub   fn bit_length(&self)->u64{
    64u64*self.len() as u64-self.leading_zeros()
 }       
    
 pub  fn remove_lead_zeros(&mut self){// Remove leading zeros
        let k = self.leading_zeros()/64u64 ; 
         self.elements.truncate(self.len() -k as usize)
       }
    
  /*  
 pub  fn remove_trail_zeros(&mut self){
       let len = self.data.iter()
 } 
 */
 pub  fn set_bit(&mut self, index: usize){ //flips the bit at the index
  
  self.elements[index/64usize]|=1<<(index%64)
 
 }
 
 pub  fn set_digit(&mut self, index: usize, value: u64){
       self.elements[index]=value;
 }
 
 pub  fn truncate(&mut self, len: usize){
     let selflen = self.len();
     let mut t = vec![];
     t.extend_from_slice( &mut self.elements[(selflen-len)..selflen]);
     self.elements=t;
      
 }
 
 pub  fn lead_digit(&self)->u64{
          self.elements[..].last().unwrap().clone()
 }
 /*
 Equality Operations 
 
 */
     
     fn is_equal(&self,other: &Mpz)->bool{// checks equality
 
         if self.len() != other.len(){
               return false
         }
   
        for (x,y) in self.elements.iter().zip(other.elements.iter()){
           if x !=y{
               return false
            }
         } 
          return true
      }
 
 
 fn less_than(&self,other: &Mpz)-> bool{
    if self.len() > other.len(){
       return false
    }
   for (x,y) in self.elements.iter().rev().zip(other.elements.iter().rev()){
      if x > y{
        return false
      }
   } 
  return true
 }
 
 fn greater_than(&self, other: &Mpz)-> bool{
    if self.len() > other.len(){
       return true
    }
    
    for (x,y) in self.elements.iter().rev().zip(other.elements.iter().rev()){
      if x < y {
       return true
      }
    }
  return true
 }
 
 /*
   Shifting operations
 
 */
 
 pub fn shift_left(&mut self, shift: usize){
    
    let mut k = self.clone();
    let mut trail_zeroes = vec![0;shift/64usize];
 
    let carry = slice_shl(&mut self.elements[..],(shift%64usize) as u32);
 
 trail_zeroes.extend_from_slice(&self.elements[..]);
 
 if carry > 0{
    trail_zeroes.push(carry)
 }
 
 self.elements = trail_zeroes;
 
 }
 
 
 pub fn shift_right(&mut self, shift: usize){
 
 let mut carry = 0u64;
 
 let mut vector : Vec<u64> = self.elements.drain((shift/64usize)..self.elements.len()).collect();
 let sub_shift = shift%64usize;
 
 for i in vector.iter_mut().rev(){
      carry = carry_shr(carry, *i,sub_shift as u32,i);
    
 }
 
 self.elements = vector;
 }
 
 pub fn shl(&self, shift: usize)->Mpz{
     let mut k = self.clone();
     k.shift_left(shift);
     k
 }
 
 pub fn shr(&self, shift: usize)->Mpz{
      let mut k = self.clone();
     k.shift_right(shift);
     k
 }
 
 /*
   Print output
 */
 
 pub  fn print(&self){
         match self.sign {
         Sign::Positive => print!("+"),
         Sign::Negative => print!("-"),
         }
       //  println!("{:?}",self.elements);
         for i in self.elements.iter().rev(){
            print!("{},",i);
            }
            
      }
 
 }
 
 impl AddIdentity for Mpz{
      
      fn add_identity()->Self{
         Mpz::new(Sign::Positive, vec![0])
      }
 }
 
 
 
 impl AddInverse for Mpz {
      
      fn add_inverse(&self)->Self{
         Mpz::new(self.sign.neg(),self.elements.clone())
      }
 }
 
 impl std::ops::AddAssign for Mpz{
 
  fn add_assign(&mut self, mut other: Self){
 
      let mut carry = 0u8;
 
     if self.sign == other.sign {
        if &self.elements.len() < &other.elements.len(){
 
            self.elements.extend_from_slice(&other.elements[self.len()..])
        }
     carry = unequal_add(&mut self.elements[..],&other.elements[..]);
     }
     
    else{
       if self.less_than(&other){
             carry = unequal_sub(&mut other.elements[..],&self.elements[..]);
             *self = other;
        }
       else {
             carry = unequal_sub(&mut self.elements[..],&other.elements[..]);
             self.elements[other.len()]-=1;
        }
    }
    
     if carry == 1u8{
          self.elements.push(1u64)
     }
   self.remove_lead_zeros();
 }
 
 }
 
 impl std::ops::Add for Mpz{
       type Output = Self;
      fn add(self, other: Self)->Self{
         let mut k =self.clone();
         k+=other;
         k
      }
 }
 
 impl MulIdentity for Mpz{
      
      fn mul_identity()->Self{
         Mpz::new(Sign::Positive, vec![1])
      }
 }
 
 impl Mul for Mpz{
    type Output = Self;
     fn mul(self,other: Self)->Self{
         let mut y= Mpz::new(self.sign,vec![]);
         let mut offset = 0usize;
        
          for i in other.elements.iter(){
              let mut k = self.scalar(i);
                  y.shift_add(&k,offset);
                  offset+=1;
         }
          y
     }
 }
 
 impl MulInverse for Mpz{
 
  //broken fix this
    fn mul_inverse(&self)->Self{
       let mut d = self.clone();
       let mut start = Mpz::new(Sign::Positive,vec![0u64;d.len()]);
       start.elements[d.len()-1]=1;
       let mut two = Mpz::new(Sign::Positive, vec![0u64;d.len()*2+1]);
       two.elements[d.len()*2]=2;
       for i in 0..20{
          let dx = d.multiply(&start);
          let delta = two.clone() + dx.add_inverse();
          start = d.multiply(&delta);
          start.truncate(d.len());
       }
       start
  }
 
 }
 
 
 impl Mpz {
 
  pub fn mersenne(x:u64)->Mpz{
      let mut k =(0..x/64).map(|_| 0xffffffffffffffff).collect::<Vec<u64>>();
          k.push((1<<x%64) -1);
    Mpz::new(Sign::Positive,k)
  }
  
  pub fn self_scalar(&mut self, x: u64){
      let  carry = scalar_slice(&mut self.elements[..],x);
     if carry > 0u64{
         self.elements.push(carry);
     }
  }
  
  pub fn scalar(&self, x: &u64)->Mpz{
      let mut k = self.clone();
      k.self_scalar(*x);
      k
  }
  
  pub fn self_div(&mut self, x: u64)->u64{
          div_slice(&mut self.elements[..],x)
  }
  
  pub fn word_div(&self,x: u64)->(Self, u64){
       let mut quotient = self.clone();
       let remainder = quotient.self_div(x);
       (quotient,remainder)
  }
  
  /*
  2 	3 	5 	7 	11 	13 	17 	19 	23 	29 	31 	37 	41 	43 	47 	53 	59 	61 	67 	71
 	73 	79 	83 	89 	97 	101
  */
    
  pub fn factor(&self)->Vec<u64>{
          //const PRIMES : [u64;24] = [3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,61,67,71,73,79,83,89,97,101];
          let primes = 100000u64.primes();  
          let mut n = self.clone();
          let mut factors = Vec::new();
          let two_factor = self.trailing_zeros();
          if two_factor > 0{
             factors.push(2u64);
             factors.push(two_factor);
             n.shift_right(two_factor as usize);
          }
  'outer :  for i in primes{
          let (quo,mut rem) = n.word_div(i);
        
         
          if rem == 0{
           let mut count = 1u64;
             factors.push(i);
             n = quo;
            
    'inner : loop {
             
             let (inner_quo, inner_rem) = n.word_div(i);
             
             if inner_rem != 0{
                break 'inner;
             }
             n = inner_quo;
             count+=1;
            
            }
            
            factors.push(count); 
          }
          
          }
          factors
  }
  
  //K-factorial
  
 pub  fn factorial(x: u64, y:u64)-> Mpz{
  
 let mut z= Mpz::mul_identity();
 
     for i in 1..x+1{
         if i%y == x%y{
            z.self_scalar(i)
         }
     }
       z
    }
    // Shifted add
    
    fn shift_add(&mut self, mut other: &Self, shift: usize){
   
   
       self.elements.resize(other.len()+shift,0);
   
       let mut carry : u8 =0;
       
       
       for i in 0..other.elements.len(){
        carry = unsafe{core::arch::x86_64::_addcarry_u64(carry,self.elements[i+shift],other.elements[i],&mut self.elements[i+shift])}
       }
   
     if carry > 0u8{
                   self.elements.push(1u64)
    }
   }
 
  pub fn multiply(&mut self,other: &Mpz)->Mpz{

          let newlen = self.len()+other.len()+1;
          let mut y  = Mpz::new(self.sign.mul(&other.sign),vec![0u64;newlen]);
         
          mul_slice2(&self.elements[..],&other.elements[..],&mut y.elements[..]);
        
          if y.elements[newlen-2]==0u64{
          y.elements.pop(); 
          y.elements.pop();
          } 
          y
     }
     
   
     // Exponentiation
  pub  fn pow(&self, mut y:u64)->Mpz{
         let mut z= Mpz::new(self.sign.pow(&y),vec![1]);
         let mut x_mpz = self.clone();
          if y == 0{
              return z
          }
         while y > 1{
    
           if y%2 ==0 {
              x_mpz = x_mpz.multiply(&x_mpz.clone());
              y>>=1;
           }
           else{
              z = x_mpz.multiply(&z);
          x_mpz = x_mpz.multiply(&x_mpz.clone());
              y = (y-1)>>1;
           }
        }
      return x_mpz.multiply(&z)
     }
     
     
 pub fn euclidean(mut self, other: Self)->(Self, Self){
     
      use std::{thread, time};
   let ten_millis = time::Duration::from_millis(100);
   
   // let mut factor = Mpz::add_identity();   // use a zero vector and setbit 
   let mut factor = Mpz::new(Sign::Positive,vec![0;self.len()-other.len()+1]);
    let mut correction = 0usize;
    while self.bit_length() > other.bit_length(){
  //  thread::sleep(ten_millis);
      let delta = (self.bit_length()-other.bit_length()-1) as usize;
     let mut k = other.shl(delta);

      if k.greater_than(&self){
   
      k=other.shl(delta-1);
 
         correction=1;
     }
   
      factor +=Mpz::mul_identity().shl(delta-correction);  
      self+=k.add_inverse();
      }
      
      if self.is_equal(&other){
      self = Mpz::add_identity();
      factor+=Mpz::mul_identity();
      }
      self.remove_lead_zeros();
      (self, factor)
    
    }
  
      //rarely works fix this
    pub fn gcd(&self, mut other: Self)->Self{
        
        use std::cmp::min;
        use std::mem::swap;
      
      let mut v = self.clone();
      let zero = Mpz::add_identity();
    
    if other.is_equal(&zero) {
          return v;
      } else if v.is_equal(&zero) {
          return other;
      }
      let i = other.trailing_zeros() as usize;  other.shift_right(i);
      let j = v.trailing_zeros() as usize;  v.shift_right(j);
      let k = min(i, j);

      loop {
         
         if other.greater_than(&v) {
             swap(&mut other, &mut v); 
         }
         v += other.add_inverse();
         v.remove_lead_zeros();
         other.remove_lead_zeros();

         if v.is_equal(&zero) || v.elements.is_empty() {
             return other.shl(k);
         }
         v.shift_right(v.trailing_zeros() as usize)
  }
        
    }
  
 
 }
 
 impl SemiRing for Mpz {}
 impl Ring for Mpz {fn characteristic()->u64 {0u64}}
  
