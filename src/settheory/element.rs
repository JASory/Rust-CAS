use std::borrow::Cow;

use crate::traits::*;
use crate::settheory::set::FiniteSet;


#[derive(Clone,PartialEq, Default)]
pub struct SetElement<'a,T> where T: Set{
    set: Cow<'a, FiniteSet<T>>,
    element: usize,
  }


pub struct MagmaElement<'a, T>{
    magma : &'a FiniteSet<T>,
    func  : fn(T,T) -> T, 
    element: usize,
}



impl<'a,T: Set> SetElement<'a, T> {
    
  pub fn new(value: &'a FiniteSet<T>, idx: usize) -> Self{
        Self{set: Cow::Borrowed(&value), element: idx}
    }
    
  pub  fn value(&self) -> T{
        self.set.elements[self.element].clone()
    }
    
  pub fn change_element(&mut self, value: usize){
       self.element = value;
  }  
}

impl<'a, T: Set> Set for SetElement<'a,T>{
  
  fn rand(&self) -> Self{
      let idx = (u64::default().rand()%self.set.card() as u64) as usize;
      let mut rng = self.clone();
      rng.change_element(idx);
      rng
      //Self::new(&self.set,idx)
  }
  
  fn format(&self) -> String{
     self.set.elements[self.element].format()
  }
}

/*
impl<'a,T: Set> MagmaElement<'a, T> {

  fn new(set: &'a FiniteSet<T>,func: fn(&T, &T) -> T)

}
*/




/*



    Construct matrix of outputs 
    
     
      
    
    Analyze matrix 
     Commutativity
     Identity
     
      
     
      
    





*/
