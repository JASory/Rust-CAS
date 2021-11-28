pub trait Set {
   fn rand()->Self;
   fn format(&self)->String;
}

pub trait Magma : Set {
    fn op(&self,other: Self)->Self;
}

pub trait Group: Magma {
    fn op_identity()->Self;
    fn op_inverse(&self)->Self;
}


pub trait AddIdentity : Magma {
    fn add_identity()-> Self;
}

pub trait AddInverse: Magma{
    fn add_inverse(&self)-> Self;
}

pub trait MulIdentity: Magma {
    fn mul_identity()-> Self;
}

pub trait MulInverse: Magma{
    fn mul_inverse(&self)-> Self;
}

pub trait Scalar{}


pub trait AdditiveGroup : std::ops::Add<Output = Self> + AddIdentity + AddInverse + Sized {}

pub trait MultiplicativeGroup : std::ops::Mul<Output = Self> + MulIdentity + MulInverse + Sized {}

pub trait SemiRing : std::ops::Add<Output = Self> + std::ops::Mul<Output = Self> + AddIdentity + MulIdentity + Sized {}

pub trait Ring : SemiRing + AddInverse { 

      fn characteristic()->u64 ;
      
      }

pub trait Integral : Ring {}

pub trait IntegralClosed: Integral {}

pub trait GCD : SemiRing {
    fn gcd(&self, other: Self)->Self;
}

pub trait UFD : GCD {
    fn irreducible(self)->bool;
    fn factor(self)->Vec<Self>;
}

pub trait PID : UFD {}

pub trait Euclidean : GCD {   // Quotient
    fn euclidean(self, other: Self)->(Self,Self);
    fn form(&self, x: Self, c: Self)->bool;
}

pub trait Field: Ring + MulInverse {}

pub trait Module : Ring + Scalar {}

pub trait VectorSpace: Field + Scalar {}

pub trait BiProduct{}

pub trait FieldAlgebra : Field + BiProduct {}

pub trait RingAlgebra : Module + BiProduct {}


pub trait PartiallyOrdered : Set {}

pub trait Lattice: PartiallyOrdered {
    
    fn infinum(&self)->Self;
    fn suprenum(&self)->Self;
}



pub trait NumberTheory {

  fn prime_omega(&self)->u64;
  fn prime_omega_small(&self)->u64;
  fn smooth(&self)->Self;
  fn is_smooth(&self, b: u64)->bool;
  fn k_free(&self,k: u64)->bool;
  fn is_square(&self)->bool;
  fn louiville(&self)->i64;
  fn mobius(&self)->i64;
  fn primorial(&self)->Self;
  fn radical(&self)->Self;
  fn factorial(&self)->Self;
  fn k_factorial(&self, k: u64)->Self;
  fn coprime(&self, x: Self)->bool;
  fn totient(&self)->Self;
  fn jordan_totient(&self, k: u32)->Self;
  fn dedekind_psi(&self)->Self;
  fn pi(&self)->u64;
  fn primes(&self)->Vec<u64>;
  fn nth_prime(&self)->Self;
  fn li(&self)->u64;
  fn mangoldt(&self)->f64;
  fn chebyshev(&self)->f64;
  
}
