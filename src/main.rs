// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. 
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, 
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

include!("mlsolver.rs");

fn main() {
    println!("Hello, mlsolver!");

    // MATLAB like matrix constructor
    let a = ml_matrix("1 2;3 4");

    // R like matrix constructor (default)
    let b = matrix(c!(1,2,3,4), 2, 2, Row);

    // Or use zeros
    let mut z = zeros(2, 2);
    z[(0,0)] = 1.0;
    z[(0,1)] = 2.0;
    z[(1,0)] = 3.0;
    z[(1,1)] = 4.0;

    // Simple but effective operations
    let c = a * b; // Matrix multiplication (BLAS integrated)

    // Easy to pretty print
    c.print();
    //       c[0] c[1]
    // r[0]     1    3
    // r[1]     2    4

    // Easy to do linear algebra
    c.det().print();
    c.inv().print();

    // and etc.

   let (a21, a01, q, r) = network1();
     
   ml_solver(&a21, &a01, &q, &r);


   println!("rows = {}, cols ={}", q.row, q.col )


}

fn network1()->(Matrix, Matrix, Matrix, Matrix) {
    //let nn =4;
    //let np = 7;
    //let mut a21 = zeros(nn,np);
    let a21 = ml_matrix("1 0 -1 -1 -1 0 0; 
                         0 1 1 0 0 -1 0; 
                         0 0 0 1 0 0 -1;
                         0 0 0 0 1 1 1");
     
       
    let mut a01 = ml_matrix("-1 -1 0 0 0 0 0");
    a01=a01.transpose();
    //demand vector 
    let mut q = ml_matrix("0.1 0.2 0.3 0.4");
    
     q = q.transpose();
    
    let mut r = ml_matrix("1.5625 50 100 12.5 75 200 100");
    r=r.transpose();

    (a21, a01, q, r) 



}
