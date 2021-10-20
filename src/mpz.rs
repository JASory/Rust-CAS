/*
 Precursor functions

*/
   use std::ops::*;
   
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
    unsafe  { core::arch::x86_64::_bextr_u64(x,0,places).overflowing_shl(64-places).0 }
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
 
 #[derive(Debug,Clone, PartialEq)]
 pub struct Mpz{
         sign: Sign,
     elements: Vec<u64>,
 }
 
 impl Set for Mpz{}
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
     
           let mut index_length : u64 =0;
    
            for i in 0..self.len(){
                if self.elements[self.len()-1usize-i]!=0u64{
                   index_length=i as u64;
                      break;
                }
            }
         self.elements[self.len()-1usize-index_length as usize].leading_zeros() as u64 + 64u64*index_length
        }
        
 pub   fn bit_length(&self)->u64{
    64u64*self.len() as u64-self.leading_zeros()
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
        }
    }
    
     if carry == 1u8{
          self.elements.push(1u64)
     }
   
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
 
 impl SemiRing for Mpz {}
 impl Ring for Mpz {}
  
