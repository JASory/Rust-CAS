
pub mod traits;
pub mod primtraits;
pub mod linal;
pub mod hypercomplex;
pub mod frac;

pub mod zring;
pub mod polynomial;

pub mod settheory;

use crate::traits::*;
use linal::*;

use crate::hypercomplex::hypercore::HyperComplex;
use crate::hypercomplex::dual::Dual;

use crate::matrix::Matrix;
use crate::sqmatrix::SqMatrix;
use crate::frac::*;

use crate::zring::zahl::ZahlRing;

use crate::settheory::set::*;
use crate::settheory::element::*;

use crate::polynomial::univariate::Univariate;



use number_theory::*;
fn main() {


       let mut z = SqMatrix::<u64,9>::unchecked_new([1,2,3,4,5,6,7,8,9]);
       // let k = Dual::new(17f64,1f64);
       println!("{}",z.format());
     z.col_swap(1,0);
     
       
        println!("{}",z.format())
}
