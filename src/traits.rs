pub trait Set {}

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

pub trait Ring : SemiRing + AddInverse {}

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
