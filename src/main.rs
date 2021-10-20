pub mod traits;
    mod numeric;
    mod rational;
    mod hypercomplex;
    mod mpz;
    mod quotient;
    
    mod matrix;
    mod blas1;
    mod blas3;
    
    use mpz::*;
    use numeric::*;      
    use traits::*;  
    use rational::*;
    use hypercomplex::*;
    use quotient::*;
    
    use blas1::*;    
    use blas3::*;
    use matrix::*;      
