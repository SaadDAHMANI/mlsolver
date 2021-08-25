// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. 
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, 
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

#[macro_use]
extern crate peroxide;
use peroxide::prelude::*;
//use peroxide::fuga::*;

pub fn ml_solver(a21 : &Matrix, a01 : &Matrix, q : &Matrix, r : &Matrix){

let nn =a21.row;
let np =a21.col;

if nn<2 {return;}
if np<1 {return;}

let mut iter : usize =0;
let itermax : usize=1; 
//let err : f64 =10f64;
//let errobj : f64 =0.0001;

 let mut _a = zeros(np,np); //A
 let mut _b =zeros(np,1); // B
 let _a12 =a21.transpose();

 println!("A12"); _a12.print();


// step 0 : compute Qmax 
let mut qmax : f64 =0.0;
for i in 0..q.row {
    qmax += q[(i,0)]; 
}


//while err>errobj {
while iter < itermax { 

  // step 1- Update A & B 

  // initilize A & B if the first iteration
  if iter==0 {
    // step 1 : establish A & B using eq.19
    // 1.1- initilize A matrix :
     initilize_a(&mut _a, &r, qmax);
  
    _a.print();
    
    // 1.2- initiliza B matrix :
        
     _b.print();      
  }
  else {


  }

  // step 2- Compute V (eq.31) and C (eq.32)
  let _inva = _a.inv();

  println!("A^-1"); _inva.print();

  let _v1 = product(&_inva, &a21.transpose());

  println!("V1") ; _v1.print();

   let _v = product(&a21, &_v1); 
  println!("V") ; _v.print();

 //let v = prouct(&a21, &inva);

 let s = ml_matrix("1.5 2.10 0 ; 4.50 3.23 -1.569");
 let t=ml_matrix("5.5 1.25; 2.3 3; 3 4");

 let mm = product(&s, &t);
 println!("mm = ") ; mm.print();
 //let expected = ml_matrix("9 13 -1; 14 13 -3; 19 18 -4");

 


  iter+=1;

}
}

fn initilize_a(result : &mut Matrix,  resistance : &Matrix, qmax : f64) {
     let np = resistance.row;
     if result.row == np {
        if result.col==np {
             for i in 0..np {
                 result[(i,i)]=resistance[(i,0)]*qmax;
             }
         }
     }
}

 fn product(left : &Matrix, right : &Matrix)->Matrix {
    
    let nrl =  left.row;
    let ncr = right.col;
    let nrr = right.row;

    let mut result = zeros(nrl,nrl);
    let mut _sum =0.0f64;
    if nrl==ncr {
        for i in 0..nrl {
            
            for j in 0..nrl{

                 _sum = 0.0f64;

                 for k in 0..nrr{

                     _sum += left[(i,k)]*right[(k,j)];

                 }

                 result[(i,j)]=_sum;
            } 
       }
    }
    result
}





//****************************************************************************************************
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn product_test1() {
         let left = ml_matrix("1 0; 2 -1");
         let right =ml_matrix("3 4; -2 -3");
         let expected = ml_matrix("3 4; 8 11");
        
         assert_eq!(product(&left, &right), expected);         
    }
    
    #[test]
    fn product_test2(){

         let s = ml_matrix("1 2 0 ; 4 3 -1");
         let t=ml_matrix("5 1; 2 3; 3 4");
         let expected = ml_matrix("9 7; 23 9");

          assert_eq!(product(&s,&t),expected);
    }

    #[test]
    fn product_test3(){
        let s = ml_matrix("1 2 0 ; 4 3 -1");
        let t=ml_matrix("5 1; 2 3; 3 4");
        let expected = ml_matrix("9 13 -1; 14 13 -3; 19 18 -4");
        
        assert_eq!(product(&t, &s),expected);        
    }

    
}
