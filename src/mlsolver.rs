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

pub fn ml_solver( a21 : &Vec<Vec<f64>>, a01 : &Vec<f64>, q : &Vec<f64>, r : &Vec<f64>, h0:f64)->Option<(Vec<f64>, Vec<f64>)> {

let nn = a21.len();
let np = a21[0].len();

if nn<2 {return Option::None;}
if np<1 {return Option::None;}

let mut iter : usize =0;
let itermax : usize=2; 
//let err : f64 =10f64;
//let errobj : f64 =0.0001;

 let mut _a = vec![vec![0.0f64; np]; np]; //A
 let mut _b =vec![0.0f64; np]; // B
 let mut _c = vec![0.0f64; np];
 let mut _flowsq = vec![0.0f64; np];
 let mut _headsh =  vec![0.0f64; nn];
 let mut _coef_a = vec![0.0f64; np]; // ai
 let mut _coef_b = vec![0.0f64; np]; //bi

 let m : f64 = 10.0;
 let n : f64 = 2.0;

 let _a12 =transpose(a21);

 print(& a21, &"A21");
 print(&_a12, &"A12");
 print_vector(&a01, "A01 :");


// step 0 : compute Qmax 
let mut qmax : f64 =0.0;
for i in 0..q.len() {
   qmax+=q[i];
}
// compute delta Q
let deltaq = qmax/m;


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
         //update ai :

         let mut _intpart : f64 =0.0;
                 
        for i in 0..np {
             _intpart=_flowsq[i]/deltaq;   
             _coef_a[i] = f64::trunc(_intpart)*deltaq;
             _coef_b[i] = f64::trunc(_intpart + f64::signum(_flowsq[i]))*deltaq;
        
              //Updating A (eq13):
             _intpart =(f64::powf(_coef_b[i], n)- f64::powf(_coef_a[i],n))/(_coef_b[i]-_coef_a[i]);
             _a[i][i]=f64::signum(_flowsq[i])*r[i]*_intpart;
            
              //Updating B (eq14):
              _b[i]=-1.0*f64::signum(_flowsq[i])*r[i]*(_intpart*_coef_a[i] - f64::powf(_coef_a[i],n));  
        } 
     }

     // Step 2 : Compute V (eq) and C 
     // Compute V:
     let inva = invers_diagonal(&_a);
     let inva = match inva {
            Ok(matrx)=>matrx,
            Err(error) => panic!("Problem with inverse diagonal matrix : {:?}", error),      
     };

     print(&inva, "[A-]");

     let _v1 = product(&a21, &inva);
     let _v1 = match _v1{
         Ok(matrx)=> matrx,
         Err(error)=> panic!("Problem with product matrices : {:?}", error),
     };

     let _v = product(&_v1, &_a12);
     let _v = match _v {
         Ok(matrx)=>matrx,
         Err(error)=> panic!("Problem with product matrices : {:?}", error),
     };

     print(&_v, "[V]");
   
     //Compute C:
     for i in 0..np {
         _c[i]=(-1.0*_b[i])-(h0*a01[i])
     }

     print_vector(&_c, "C : ");

     // Step 3 : Compute H (eq.29)
     let invv = invers(&_v);
     
     let invv = match invv{
         Ok(matrix) => matrix,
         Err(error) => panic!("Problem with inverse matrix : {:?}", error),
        
     };

     let tmp = product2(&_v1, &_c);

     let mut tmp = match tmp {
         Ok(vectr) => vectr,
         Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
     };

     for i in 0..nn {
         tmp[i] -= q[i];   
     }

     let _h = product2(&invv, &tmp);
     _headsh = match _h {
         Ok(vect)=>vect,
         Err(error) =>  panic!("Problem with product matrix by vector : {:?}", error),
     };

     print_vector(&_headsh, "[H] :");

     // Step 4 : Compute flowws Q (eq30)
      let tmpql = product2(&inva, &_c);
      let tmpql =match tmpql{
          Ok(vect)=>vect,
          Err(error) =>  panic!("Problem with product matrix by vector : {:?}", error),
      };
      
      let tmpqm = product(&inva, &_a12);
      let tmpqm =match tmpqm {
          Ok(matrx) => matrx,
          Err(error) => panic!("Problem with matrix multiplication : {:?}", error),
      };

      let tmpqr = product2(&tmpqm, &_headsh);
      let tmpqr = match tmpqr {
            Ok(vect) => vect,
            Err(error) => panic!("Problem with product matrix by vector : {:?}", error),        
      };

      for i in 0..np {
          _flowsq[i]=tmpql[i]-tmpqr[i];
      }

      print_vector(&_flowsq, "[Q]");

     iter+=1;
     }

        Some((_flowsq, _headsh))
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


fn product2(left : &Vec<Vec<f64>>, right : &Vec<f64>)-> Result<Vec<f64>, String> {
    
    let m =  left.len();
    let pl = left[0].len();


    let pr = right.len();

    let mut result = vec![0.0f64; m];
    let mut _sum =0.0f64;
    if pl==pr {
        for i in 0..m{     

            _sum = 0.0f64;

            for j in 0..pl{                          
            _sum += left[i][j]*right[j];     
            } 

            result[i]=_sum;
       }
       Ok(result)
    }
    else{
        Err(String::from("Colomns's count of left matrix not equals rows's count of right vector!"))
    }
    
}



fn invers(matrix : &Vec<Vec<f64>>)->Result<Vec<Vec<f64>>, String> {
    if matrix.len() != matrix[0].len() {
        Err(String::from("Matrix is not square!"))
    }
    else {
        let n = matrix.len();
        //let mut inv = vec![vec![0.0f64; n]; n];
        //Using peroxide crate :

        let mut pmatrix = zeros(n,n);
        //copy matrix 
        for i in 0..n {
            for j in 0..n {
                pmatrix[(i,j)]=matrix[i][j];
            }
        }
        let inversed =pmatrix.inv().to_vec(); 
        Ok(inversed)
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

         let mut expected = vec![vec![0.0f64; 2]; 2];
         expected[0][0]=3.0;
         expected[0][1]=4.0;
         expected[1][0]= 8.0;
         expected[1][1]= 11.0;
         
         let expected : Result<Vec<Vec<f64>>, String> = Ok(expected);
        
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
    #[test]
    fn product2_test1() {
     
        let mut left = vec![vec![0.0f64; 3]; 2];
        left[0][0]=1.0;
        left[0][1]=-1.0;
        left[0][2]=2.0;

        left[1][0]=0.0;
        left[1][1]=-3.0;
        left[1][2]= 1.0;

        let mut vect = vec![0.0f64; 3];
        vect[0]=2.0;
        vect[1]=1.0;
        vect[2]=0.0;

        let mut expected = vec![0.0f64; 2];
        expected[0]=1.0;
        expected[1]=-3.0;

        let expect : Result<Vec<f64>, String> = Ok(expected);

        assert_eq!(product2(&left, &vect), expect);


    }

       
}
