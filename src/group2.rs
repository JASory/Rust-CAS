/*

  

*/
use std::marker::PhantomData;


  #[derive(Clone)]
 pub struct Additive;
 
  #[derive(Clone)]
 pub struct Multiplicative;
 
  #[derive(Clone)]
 pub struct BinaryFunc;
 
 
pub trait Op: Clone {}

pub trait Magma<O: Op>{}   

pub trait Group<O: Op> {

} 


impl Op for Additive {}
impl Op for Multiplicative {}

impl Op for BinaryFunc {}

trait LeftInverse<O: Op>{
  fn left_inverse(&self) -> Self;
}

trait RightInverse<O: Op>{
  fn right_inverse(&self) -> Self;
}

trait Inverse<O:Op>{
  fn inverse(&self) -> Self;
}

trait LeftIdentity<O: Op>{
  fn left_identity(&self) -> Self;
}

trait RightIdentity<O: Op>{
  fn right_identity(&self) -> Self;
}

trait Identity<O: Op>: LeftIdentity<O> + RightIdentity<O>{
  fn identity(&self) -> Self;
}
/*
impl Identity<Additive> for u64{
  fn identity(&self) -> Self {0u64}
}

impl Identity<Multiplicative> for u64{
  fn identity(&self) -> Self {1u64}
}
*/

macro_rules! impl_left_ident {
    ($M:ty; $V:expr; $($T:ty),* $(,)*) => {
        $(impl LeftIdentity<$M> for $T { #[inline] fn left_identity(&self) -> $T {$V} })+
    }
}


macro_rules! impl_right_ident {
    ($M:ty; $V:expr; $($T:ty),* $(,)*) => {
        $(impl RightIdentity<$M> for $T { #[inline] fn right_identity(&self) -> $T {$V} })+
    }
}

macro_rules! impl_ident {
    ($M:ty; $V:expr; $($T:ty),* $(,)*) => {
        $(impl Identity<$M> for $T { #[inline] fn identity(&self) -> $T {$V} })+
    }
}

impl_left_ident!(Additive;0;u64);
impl_left_ident!(Multiplicative;1;u64);

impl_right_ident!(Additive;0;u64);
impl_right_ident!(Multiplicative;1;u64);

impl_ident!(Additive;0;u64);
impl_ident!(Multiplicative;1;u64);

#[repr(C)]
#[derive(Debug)]
pub struct Id<O: Op = Multiplicative> {
    _op: PhantomData<O>,
}

impl<O: Op> Id<O> {
    /// Creates a new identity element.
    #[inline]
    pub fn new() -> Id<O> {
        Id { _op: PhantomData }
    }
}

impl<O: Op> RightIdentity<O> for Id<O> {
    #[inline]
    fn right_identity(&self) -> Id<O> {
        Id::new()
    }
    
}

impl<O: Op> LeftIdentity<O> for Id<O> {
    #[inline]
    fn left_identity(&self) -> Id<O> {
        Id::new()
    }
    
}

impl<O: Op> Identity<O> for Id<O> {
    #[inline]
    fn identity(&self) -> Id<O> {
        Id::new()
    }
    
}

fn main(){

 println!("{}",<u64 as Identity<Additive>>::identity(&2u64))
}




