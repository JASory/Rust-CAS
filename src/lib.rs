
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
