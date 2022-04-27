
pub mod traits;
pub mod primtraits;
pub mod linal;
pub mod hypercomplex;
pub mod frac;
pub mod mpf;

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

use crate::mpf::Mpf;

use number_theory::*;




 
 fn file_write(x: &str, linelen:usize, filename: &str){
 
   use std::fs::File;
   use std::io::Write;
 
    let xbytes = x.as_bytes();
    let len = xbytes.len();
    let mut result = vec![];
     for i in 0..1003200{
     if i%linelen == 0 && i > 0{
              result.push(10);
                            result.push(10);
            }
             result.push(xbytes[i])
           }
           
  let mut file = File::create(filename).expect("Whoopsie, ran out of paper . . .");

   file.write_all(&result).expect("Whoopsie ran out of ink . . .");
           
     }
 
 fn read_write(x: &str, out: &str){
 use std::fs::File;
use std::io::{self, prelude::*, BufReader};
 
     let file = File::open(x).expect("Not a file");
    let reader = BufReader::new(file);
    let mut k = vec![];
    for line in reader.lines() {
        k.push(line.unwrap())
        //println!("{}", line?);
    }
    let fused = k.join("");
    
    
    
    file_write(&fused,80,out);
 }


fn main() {


      /*
   for i in 788..1000u64{
     if i.is_prime(){
     let num = i.to_string();
     println!("Sqrt{}",i);
     let k = Mpf::from_string(&(num.clone()+".0"));
     let sqrt = k.sqrt(111_1111);
     file_write(&sqrt.to_string(),80,&("sqrt".to_owned() + &num))
     }
     }
     */
     
    // let input = "/home/jasory/Projects/CAS6/sqrt/sqrt".to_string();
     //let output = "/home/jasory/sqrt/sqrt".to_string();
     /*
     let slash = r#"\"#;
     let chap = "chapter{Sqrt of ".to_string();
     let close = "}";
     let input = "input{sqrt";
     
     for i in 1..1000u32{
       if i.is_prime(){
       println!("{}{}{}{}",slash,chap,i,close);
       println!("{}{}{}{}",slash,input,i,close);
       */
       /*
       let time = std::time::Instant::now();
       let mut count = 0u32;
       for i in 0..u32::MAX{
         if i.is_prime(){
           count+=1;
         }
         if i%20_000_000u32==0u32{//288230376151711744
           println!("{}",count);
           count+=0u32;
         }
       }
       */
   //read_write(&(input.clone()+&i.to_string()),&(output.clone()+&i.to_string()))
    
   //}
     //let product = sqrt.product(&sqrt);
      //    file_write(&product.to_string(),40,"sqrt2")
    //let product = k.product(&k);   
       // println!("{}",sqrt.to_string())
       /*
       let p = Mpf::from_string("2.3");
       let z = p.sqrt(1111);
       println!("{}",z.to_string());
       */
       
       let k = Mpf::from_string("-3.1415926535897932384626433832795028841971693993751058209749445923");
       let q = Mpf::from_string("1.20567");
       
       let p = k.division(&q,60);
       println!("{}",p.to_string());
       }
//}
