// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. 
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, 
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

//#[macro_use]
extern crate peroxide;
use peroxide::prelude::*;
//use peroxide::fuga::*;

pub fn ml_solver( a21 : &Vec<Vec<f64>>, a01 : &Vec<f64>, q : &Vec<f64>, r : &Vec<f64>){

let nn = a21.len();
let np = a21[0].len();

if nn<2 {return;}
if np<1 {return;}

let mut iter : usize =0;
let itermax : usize=1; 
//let err : f64 =10f64;
//let errobj : f64 =0.0001;

 let mut _a = vec![vec![0.0f64; np]; np]; //A
 let mut _b =vec![0.0f64; np]; // B
 let _a12 =transpose(a21);

 print(& a21, &"A21");
 print(&_a12, &"A12");


// step 0 : compute Qmax 
let mut qmax : f64 =0.0;
for i in 0..q.len() {
   qmax+=q[i];
}


//while err>errobj {
while iter < itermax { 

  // step 1- Update A & B 

  // initilize A & B if the first iteration
  if iter==0 {
    // step 1 : establish A & B using eq.19
    // 1.1- initilize A matrix :
     initilize_a(&mut _a, & r, qmax);
  
     print(&_a,&"[A]");
    
    // 1.2- initiliza B matrix :
        
      print_vector(&_b, &"[B]");      
  }
  else {


  }


     iter+=1;
     }
}

fn initilize_a(result : &mut Vec<Vec<f64>>,  resistance : &Vec<f64>, qmax : f64) {
    
    let np = resistance.len();
     if result.len() == np {
        if result[0].len()==np {
             for i in 0..np {
                 result[i][i]= resistance[i]*qmax;
             }
         }
     }
}


fn transpose(matrix: &Vec<Vec<f64>>)-> Vec<Vec<f64>> {
 
    let nr = matrix.len();
    let nc = matrix[0].len();
    let mut transposed = vec![vec![0.0f64; nr]; nc];

    for i in 0..nr{
        for j in 0..nc {
            transposed[j][i]=matrix[i][j];
        }
    }

    transposed
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


fn print( matrix : &Vec<Vec<f64>>, msg: &str) {

    let nr=matrix.len();
    let nc = matrix[0].len();

    println!("---- {}", msg);
    for i in 0..nr {
         print!("[{},:]",i);
        for j in 0..nc {
           print!(" {}", matrix[i][j]); 
        }
        println!(" ");
    } 
}

fn print_vector( vector : &Vec<f64>, msg: &str) {

    let nr=vector.len();
    println!("---- {}", msg);
    for i in 0..nr {
         print!("[{},:]",i);
         println!("  {}",vector[i]); 
    }
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
