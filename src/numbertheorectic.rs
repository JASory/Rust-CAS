/*

 Number Theorectic functions not included in other Integral domains, 
   look towards integrating into all Integral domains if possible

*/

use crate::traits::*;
use crate::numeric::*;

impl NumberTheory for u64{

   fn prime_omega(&self)->u64{
       let k = self.factor();
       let mut count = 0u64;
       if self == &1 {return count}
       for i in k[1..].iter().step_by(2){
         count+=i
       }
       count
   }
   
   fn prime_omega_small(&self)->u64{
        (self.factor().len()/2usize) as u64
   }
   
   fn smooth(&self)->u64{
       let k = self.factor();
       k[k.len()-2]
   }
   
   fn is_smooth(&self,b: u64 )->bool{
        self.smooth()==b
   }
   
   fn k_free(&self, k: u64)->bool{
       let factors = self.factor();
      for i in 0..factors.len(){
        if factors[i] == k{
          if i == 0{
            ();
          }
          else{
            return false
          }
        }
      }
      return true
   }
   
   fn is_square(&self)->bool{
      ((*self as f64).sqrt().round() * (*self as f64).sqrt().round()) as u64 == *self
   }
   /*
   fn legendre(&self, n: )->bool{
   
   }
   */
   fn louiville(&self)->i64{
       if self.prime_omega()&1 ==0{
         return 1
       }
       else{return -1}
   }
   
   fn mobius(&self)->i64{
       if self.k_free(2)==false{
         return 0
       }
       else{
          self.louiville()
            
         }
   }
   
   fn primorial(&self)->u64{
      self.primes().iter().product::<u64>()
   }
   
   fn radical(&self)->u64{
       self.factor().iter().step_by(2).product::<u64>()
   }

   fn factorial(&self)->u64{
      const FACTORIALS : [u64;21] = [
      1,1,2,6,24,120,720,5040,40320,362880,3628800,39916800,479001600,
      6227020800, 87178291200, 1307674368000, 20922789888000,355687428096000,
       6402373705728000, 121645100408832000, 2432902008176640000 
      ]; 
      FACTORIALS[(*self) as usize ]
  }
  
  fn k_factorial(&self, k: u64)->u64{
      let mut product = 1u64; 
      for i in 1..*self{
       if i%k == *self%k{
        product*=i
        }
      }
      product
  }


   fn coprime(&self,other: Self)->bool{
       self.gcd(other) == 1
   }
   
   fn totient(&self)->u64{
       let factors = self.factor();
       let numerator = factors.iter().step_by(2).map(|x| x -1u64).product::<u64>();
       let denominator = factors.iter().step_by(2).product::<u64>();
       (self/denominator)*numerator
   }   
      
   fn jordan_totient(&self,k: u32)->u64{
       let factors = self.factor();
       let mut z = self.pow(k) as f64;
       for i in factors.iter().step_by(2){
         z*=(1.0-(i.pow(k) as f64 ).recip());
       }
      z.round() as u64
   }
   
   fn dedekind_psi(&self)->u64{
       let factors = self.factor();
       let mut z = *self as f64;
       for i in factors.iter().step_by(2){
         z*=(1.0+(*i as f64 ).recip());
       }
      z.round() as u64
   
   }   
      
   fn pi(&self)->u64{
       let mut count = 0;
       let upper = *self;
       for i in 0..upper{
        if i.irreducible(){
          count+=1;
        }
       }
       count
    }
    
   fn primes(&self)->Vec<u64>{
       let limit = self.clone() as usize;
    if limit < 3 {
        return if limit < 2 { vec![] } else { vec![2] }
    }
 
    let ndxlmt = (limit - 3) / 2 + 1;
    let bfsz = ((limit - 3) / 2) / 32 + 1;
    let mut cmpsts = vec![0u32; bfsz];
    let sqrtndxlmt = ((limit as f64).sqrt() as usize - 3) / 2 + 1;
 
    for ndx in 0..sqrtndxlmt {
        if (cmpsts[ndx >> 5] & (1u32 << (ndx & 31))) == 0 {
            let p = ndx + ndx + 3;
            let mut cullpos = (p * p - 3) / 2;
            while cullpos < ndxlmt {
                unsafe { // avoids array bounds check, which is already done above
	            let cptr = cmpsts.get_unchecked_mut(cullpos >> 5);
	            *cptr |= 1u32 << (cullpos & 31);
                }
//                cmpsts[cullpos >> 5] |= 1u32 << (cullpos & 31); // with bounds check
                cullpos += p;
            }
        }
    }
 
    (-1 .. ndxlmt as isize).into_iter().filter_map(move |i| {
                if i < 0 { Some(2) } else {
                    if cmpsts[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                        Some((i + i + 3) as u64) } else { None } }
    }).collect::<Vec<u64>>()
}

  fn nth_prime(&self)->u64{
      let factors = (self*100).primes();
      factors[*self as usize-1usize]
      
  }
  
  fn li(&self)->u64{
        let x = self.clone() as f64;
      (x *  (0..x.log10() as u64).map(|k| 
      k.factorial() as f64 * x.log10().powf((-(k as f64)-1f64))   ).sum::<f64>() ) as u64
  }
  
  fn mangoldt(&self)->f64{
      let k = self.factor();
      if k.len() == 2{
        return (k[0] as f64).log10()
      }
      return 0f64
  }
  
  fn chebyshev(&self)->f64{
      let k = self.primes();
      let mut sum = 0f64;
      for i in k {
        sum+=(i as f64).log10();
      }
      sum
  }
  
  
   }    
