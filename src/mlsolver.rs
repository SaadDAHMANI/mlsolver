// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. 
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, 
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

//#[macro_use]
//extern crate peroxide;
//use peroxide::prelude::*;
//use peroxide::fuga::*;

pub fn ml_solver( a21 : &Vec<Vec<f64>>, a01 : &Vec<f64>, q : &Vec<f64>, r : &Vec<f64>, h0:f64){

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
 let mut _c = vec![0.0f64; np];
 let _a12 =transpose(a21);

 print(& a21, &"A21");
 print(&_a12, &"A12");
 print_vector(&a01, "A01 :");


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
     else { }

    // Step 2 : Compute V (eq) and C 

     let inva = invers_diagonal(&_a);
     let inva = match inva {
            Ok(matrix)=>matrix,
            Err(error) => panic!("Problem with inverse diagonal matrix : {:?}", error),      
     };

     print(&inva, "[A-]");

     let _v1 = product(&inva, &_a12);
     let _v1 = match _v1{
         Ok(matrix)=> matrix,
         Err(error)=> panic!("Problem with product matrices : {:?}", error),
     };

     let _v = product(&a21, &_v1);
     let _v = match _v {
         Ok(matrix)=>matrix,
         Err(error)=> panic!("Problem with product matrices : {:?}", error),
     };

     print(&_v, "[V]");
     //print(&_v, "[V]");

     for i in 0..np {
         _c[i]=(-1.0*_b[i])-(h0*a01[i])
     }

     print_vector(&_c, "C : ");






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


 fn product(left : &Vec<Vec<f64>>, right : &Vec<Vec<f64>>)-> Result<Vec<Vec<f64>>, String> {
    
    let m =  left.len();
    let pl = left[0].len();

    let n = right[0].len();
    let pr = right.len();

    let mut result = vec![vec![0.0f64; n]; m];
    let mut _sum =0.0f64;
    if pl==pr {
        for i in 0..m{          
            
            for j in 0..n{

                _sum = 0.0f64;

                 for k in 0..pl{

                    _sum += left[i][k]*right[k][j];

                 }

                 result[i][j]=_sum;
            } 
       }
       Ok(result)
    }
    else{
        Err(String::from("Colomns's count of left matrix not equals rows's count of right matrix!"))
    }
    
}

 //fn product2(value : f64, vector : &Vec<f64>)->Vec<f64> {
    
    //let mut result = vec![0.0f64; vector.len()];
    //for i in 0..vector.len() {
     //    result[i]=value*vector[i];
    // }
     //result
 //}

fn invers_diagonal(matrix : &Vec<Vec<f64>>)-> Result<Vec<Vec<f64>>, String> {
    
    if matrix.len()==0 {
        Err(String::from("The matrix size must be >0!"))
     }
     else {
    
         if matrix.len()== matrix[0].len() {
             let mut invers = vec![vec![0.0f64; matrix.len()]; matrix.len()];

             for i in 0.. matrix.len(){
                 invers[i][i]=1.0/matrix[i][i];
             }

      Ok(invers)

    }
    else {
         Err(String::from("The matrix is not square!"))
    }
  }   
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

         let mut left = vec![vec![0.0f64; 2]; 2];
         left[0][0]=1.0;
         left[1][0]=2.0;
         left[1][1]=-1.0;

         let mut right = vec![vec![0.0f64; 2]; 2];
         right[0][0]=3.0;
         right[0][1]=4.0;
         right[1][0]=-2.0;
         right[1][1]= -3.0;
         
         let expected = [[3.0, 4.0], [8.0, 11.0]];
        
         assert_eq!(product(&left, &right), expected);         
    }
    
    #[test]
    fn invers_diagonal_test1() {
         let mut mtrx = vec![vec![0.0f64; 2]; 2];
         mtrx[0][0]=2.0;
         mtrx[1][1]=-2.0;
         let mut invrs = vec![vec![0.0f64; 2]; 2];
         invrs[0][0]=0.5;
         invrs[1][1]=-0.5;
         
         let expected : Result<Vec<Vec<f64>>, String> = Ok(invrs);

         assert_eq!(invers_diagonal(&mtrx), expected);
    }

    #[test]
    fn invers_diagonal_when_fail(){
        let mtrx = vec![vec![0.0f64; 2]; 3];
        let expected : Result<Vec<Vec<f64>>, String> = Err(String::from("The matrix is not square!"));
        assert_eq!(invers_diagonal(&mtrx), expected);

    }

       
}
