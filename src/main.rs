// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. 
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, 
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

include!("mlsolver.rs");

fn main() {
    println!("Hello, mlsolver!");

   let (a21, a01, q, r) = network1();
     
   ml_solver(&a21, &a01, &q, &r);
  


}

fn network1()-> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {

    let nn =4;
    let np = 7;
    let mut a21 = vec![vec![0.0f64; np]; nn];
    
    a21[0][0] =1.0f64;     
    a21[0][2] =-1.0f64;
    a21[0][3] =-1.0f64;
    a21[0][4] =-1.0f64;

    a21[1][1] =1.0f64;
    a21[1][2] =1.0f64;
    a21[1][5] =-1.0f64;

    a21[2][3] =1.0f64;
    a21[2][6] =-1.0f64;

    a21[3][4] =1.0f64;
    a21[3][5] =1.0f64;
    a21[3][6] =1.0f64;   
    
    
    //let mut a01 = ml_matrix("-1 -1 0 0 0 0 0");
    let mut a01 = vec![0.0f64; np];
    a01[0]=-1.0f64;
    a01[1]=-1.0f64;

    //let mut q = ml_matrix("0.1 0.2 0.3 0.4");
    let mut q = vec![0.0f64; nn];
    q[0]=0.1;
    q[1]=0.2;
    q[2]=0.3;
    q[3]=0.4;

    //let mut r = ml_matrix("1.5625 50 100 12.5 75 200 100");

    let mut r = vec![0.0f64; np];
    r[0]=1.5625;
    r[1]=50.0;
    r[2]=100.0;
    r[3]=12.50;
    r[4]=75.0;
    r[5]=200.0;
    r[6]=100.0;

    (a21, a01, q, r) 
 
}

