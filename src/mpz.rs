/*
 Precursor functions

*/

use crate::numeric::Sign;
use crate::traits::*;

   // In-place Multiply-add carry
fn mul_carry(carry: u64, x: u64, y: &u64, output: &mut u64)->u64{

    let product = (x as u128 * *y as u128) + carry as u128 ;

        *output=product as u64;
          (product>>64) as u64
   }
   
   
   fn mul_carry2(carry: u64, x: u64, y: u64, output: &mut u64)->u64{

let product = (x as u128 * y as u128) + carry as u128 + *output as u128;

*output=product as u64;

  (product>>64) as u64
}
  



   // In-place shift-right, returns the extra bits that were shifted out
fn carry_shr(carry: u64, x: u64, places: u32, output: &mut u64)->u64{
    *output = (x>>places)|carry ;
    unsafe  { core::arch::x86_64::_bextr_u64(x,0,places)<<(64-places) }
}

   // In-place shift-left, returns the extra bits that were shifted out
fn carry_shl(carry: u64, x: u64, places: u32, output: &mut u64)->u64{
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
     carry = mul_carry(carry,*i,&scale, i);
   }   
   carry
  }



  

 /*
 Mpz struct
 
 */
 
 #[derive(Debug,Clone)]
 pub struct Mpz{
         sign: Sign,
     elements: Vec<u64>,
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
     
           let mut index_length : u64 =0;
    
            for i in 0..self.len(){
                if self.elements[self.len()-1usize-i]!=0u64{
                   index_length=i as u64;
                      break;
                }
            }
         self.elements[self.len()-1usize-index_length as usize].leading_zeros() as u64 + 64u64*index_length
        }
 
 pub   fn trailing_zeros(&self)-> u64{// Trailing zeros
          let mut index_length : u64 =0;
    
         for i in 0..self.len(){
             if self.elements[i] !=0u64{
                 index_length=i as u64;
                    break;
             }
         }
         self.elements[index_length as usize].trailing_zeros() as u64+ 64u64*index_length
       }
    
 pub  fn remove_lead_zeros(&mut self){// Remove leading zeros
        let k = self.leading_zeros()/64u64 ; 
        self.elements.truncate(k as usize);
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
        }
    }
    
     if carry == 1u8{
          self.elements.push(1u64)
     }
   
 }
 
 }
 
 impl MulIdentity for Mpz{
      
      fn mul_identity()->Self{
         Mpz::new(Sign::Positive, vec![1])
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
         let mut y= Mpz::new(self.sign.mul(&other.sign),vec![]);
         let mut offset = 0usize;
        
          for i in other.elements.iter(){
              let mut k = self.scalar(i);
                  y.shift_add(&k,offset);
                  offset+=1;
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
 
  }
