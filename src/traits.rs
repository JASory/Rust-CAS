    
pub trait Set : PartialEq + Default + Clone {
   fn rand(&self)->Self;
   fn format(&self)->String;
}

pub trait Magma : Set {
    fn op(&self,other: &Self)->Self;
}

pub trait SemiGroup : Magma {
     fn asso_op(&self, other: &Self)->Self;
}

pub trait Monoid : SemiGroup {
     fn op_identity(&self)->Self;
}

pub trait Group: Monoid {
    fn op_identity(&self)->Self;
    fn op_inverse(&self)->Self;
}

pub trait AddOp : Magma{
   fn addition(&self, other: &Self) -> Self;
}

pub trait AddIdentity : Magma {
    fn add_identity(&self)-> Self;
}

pub trait AddInverse: Magma{
    fn add_inverse(&self)-> Self;
}

pub trait MulOp : Magma{
   fn product(&self, other: &Self) -> Self;
}

pub trait MulIdentity: Magma {
    fn mul_identity(&self)-> Self;
}

pub trait MulInverse: Magma{
    fn mul_inverse(&self)-> Self;
} 



pub trait AdditiveGroup : AddOp + AddIdentity + AddInverse + Sized {}

pub trait MultiplicativeGroup : MulOp + MulIdentity + MulInverse + Sized {}

pub trait SemiRing : AddOp + MulOp + AddIdentity + MulIdentity + Sized {}

pub trait Rng : AddOp + MulOp + AddIdentity + AddInverse + Sized {}

pub trait Ring : SemiRing + AddInverse { 

      fn characteristic(&self)->u64 ;
      
      }
      
pub trait CommutativeRing : Ring {}      
      // Start Field chain
pub trait Integral : CommutativeRing {}

pub trait IntegralClosed: Integral {}

pub trait GCD : IntegralClosed {
    fn gcd(&self, other: &Self)->Self;
}

pub trait UFD : GCD {
    fn irreducible(&self)->bool;
    fn factorization(&self)->Vec<Self>;
}

pub trait PID : UFD {}

pub trait Euclidean : PID {   // Quotient
    fn euclidean(&self, other: &Self)->(Self,Self);
}

pub trait Field: Ring + MulInverse {}      

// Boolean Ring

pub trait BooleanRing : CommutativeRing {}

 // Further work needed for all below
pub trait LieRing : Ring {}
 
pub trait KleeneAlgebra : SemiRing {}
