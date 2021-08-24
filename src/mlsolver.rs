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


//while err>errobj {
while iter < itermax { 

  // step 1- Update A & B
  if iter==0 {
    // step 1 : establish A & B using eq.19
    // 1.1- initilize A matrix :
    let a= initilize_a(&r, qmax);
  
    //a.print();
    
    // 1.2- initiliza B matrix :
    let b = zeros(np,1);
    
    b.print();      
  }
  else {


  }

  // step 2- Compute V (eq.31) and C (eq.32)
  
  let inva= a.transpose().inv();

  //let v = inv_a*a21;

  println!("[A]^-1 = a.inv()");
  inva.print();

 //let v = prouct(&a21, &inva);

 


  iter+=1;

}





}

fn initilize_a(resistance : &Matrix, qmax : f64)->Matrix {
      let np = resistance.row;
     let mut a = zeros(np, np);
     for i in 0..np {
        a[(i,i)]=resistance[(i,0)]*qmax;    
     }

    return a;
}


fn product(left : &Matrix, right : &Matrix)->Matrix{

    //let left = ml_matrix("1 0; 2 -1");
    //let right =ml_matrix("3 4; -2 -3");
    //result = ml_matrix("3 4; 8 11");


    //let s = ml_matrix("1 2 0 ; 4 3 -1");
    //let t=ml_matrix("5 1; 2 3; 3 4");
    //let z=product(&s,&t);
    //    

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
