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



// step 0 : compute Qmax 
let mut qmax : f64 =0.0;
for i in 0..q.row {
    qmax += q[(i,0)]; 
}

// step 1 : establish A & B using eq.19
// 1.1- initilize A matrix :
let a= initilize_a(&r, qmax);

a.print();

// 1.2- initiliza B matrix :
let b = zeros(np,1);

b.print();


//while err>errobj {
while iter < itermax {
    iter+=1;

  // step 1- Update A & B

  // step 2- Compute V (eq.31) and C (eq.32)
  
  let inva= a.transpose().inv();

  //let v = inv_a*a21;

  println!("[A]^-1 = a.inv()");
  inva.print();

  let v = prouct(&a21, &inva);


}





}

fn initilize_a(resistance : &Matrix, qmax : f64)->Matrix {
   
    let np = resistance.row;

    let mut a = eye(np);
 
    for i in 0..np {
        a[(i,i)]=resistance[(i,0)]*qmax;    
    }

    return a;
}


fn prouct(left : &Matrix, right : &Matrix)->Matrix{

    let nr =  left.row;
    let result = zeros(nr,nr);
    result

}
