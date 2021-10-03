
pub mod traits;
    mod numeric;
    mod rational;
    mod hypercomplex;
    mod mpz;
    use mpz::*;
    use numeric::*;      
    use traits::*;  
    use rational::*;
    use hypercomplex::*;
                



fn main() {
     let mut a = HyperComplex::<f64,4>::unchecked_new([0f64,2f64,2f64,3f64]);
     let p = a.multiply(a.clone());
     println!("{:?}",p )
     
   
    
   
}
