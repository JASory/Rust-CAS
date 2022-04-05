



 #[derive(Clone,Default, PartialEq)]
pub struct FiniteSet<T>{
    pub(crate) elements: Vec<T>,
}

impl<T> FiniteSet<T>{

 pub fn unchecked_new(elements: Vec<T>)->Self{
     Self{elements}
 }
      // cardinality of set
 pub fn card(&self) -> usize{
     self.elements.len()
 }
 
 pub fn clear(&mut self){
     self.elements.clear();
 }
 
 pub fn empty() -> Self{
    Self{elements: vec![]}
 }

}


impl<T: Clone + PartialEq> FiniteSet<T>{

 pub fn complement(&mut self, other: &Self)-> Self{  //subtracts argument from self
       let mut k = self.clone();
     for y in &other.elements{
        k.elements.retain(|x| x !=y)
     }
    k
}


 pub fn set_union(&self, other: &Self)-> Self{
    let mut y = Self::unchecked_new(vec![]);
    
    for p in self.elements.iter().chain(other.elements.iter()){
       if !y.elements.iter().any(|x| x==p){
            y.elements.push(p.clone())
       }
    }
   y
  }


pub fn intersection(&self,other:&Self)->Self{
    let mut y = Self::unchecked_new(vec![]);
    
     for p in &other.elements{
      if self.elements.iter().any(|x| x ==p){
       y.elements.push(p.clone())
      }
     }
     y
   }
   
 pub fn jaccard(&self, other: &Self)-> (f64,f64){
      let index = self.intersection(other).card() as f64/(self.card() + other.card() - self.intersection(other).card()) as f64;
      (index, 1.0-index)
   }
   
   // cartesian product
 pub  fn cartesian(&mut self, other: &Self)-> FiniteSet<(T,T)>{
      let mut cartset= FiniteSet::<(T,T)>::unchecked_new(vec![]);
       for i in &self.elements{
          for j in &other.elements{
              cartset.elements.push((i.clone(),j.clone()))
          }
       }
       cartset
     }   
     
     // Generate set from f(x) where x \in Z
     
 pub fn gen_z(func: fn(&i64)-> T, infimum: i64, supremum: i64)->Self{
       Self::unchecked_new( (infimum..supremum).map(|x| func(&x)).collect::<Vec<T>>())
  }
  
     // Generate set from elements of self
 pub fn set_gen(&self, func: fn(&T)-> T)->Self{
      let mut k = vec![];
      for i in &self.elements{
         k.push(func(i))
      }
      Self::unchecked_new(k)
  } 
  
  
     // Check if f(x) -> x where x \in self
 pub fn is_unary_closed(&self, func: fn(&T) -> T) -> bool{
     for i in &self.elements{
       if !self.elements.contains(&func(i)){
         return false
       }
     }
     return true
 }
 
 
 
 
 pub fn binary_table(&self, func: fn(&T,&T) -> T) -> Vec<T> {
      let mut k = vec![];
      for i in &self.elements{
        for j in &self.elements{
          k.push(func(i,j))
        }
      }
      k
 }
 
 
}
