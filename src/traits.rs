pub trait AddIdentity{
    fn add_identity()->Self;
}

pub trait AddInverse{
    fn add_inverse(&self)->Self;
}

pub trait AddOperation{
    fn addition(self, other: Self)->Self;
}

pub trait MulIdentity{
    fn mul_identity()->Self ;
}

pub trait MulInverse{
   fn mul_inverse(&self)->Self;
}

pub trait MulOperation{
   fn multiply(self, other: Self)->Self;
}

pub trait AdditiveGroup: AddIdentity + AddInverse + AddOperation {}

pub trait MultiplicativeGroup: MulIdentity + MulInverse + MulOperation {}

pub trait SemiRing: AddIdentity + MulIdentity + AddOperation + MulOperation {}

pub trait Ring: SemiRing + AddInverse {} 

pub trait Field: AdditiveGroup + MultiplicativeGroup {}


pub trait UFD: SemiRing {
    
    fn irreducible(self)->bool;
}



pub trait EuclideanDomain {
    fn remainder(self,other: Self)->Self;
    fn form(&self, x: Self, c: Self)->bool;
    fn gcd(self, other: Self)->Self;
}
