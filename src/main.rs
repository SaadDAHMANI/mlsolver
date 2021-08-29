// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. 
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, 
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

use std::time::Instant;
include!("network.rs");
include!("mlsolver.rs");
include!("node.rs");
include!("link.rs");
include!("junction.rs");
include!("tank.rs");
include!("reservoir.rs");
include!("pipe.rs");
include!("pump.rs");
//include!("valve.rs");


fn main() {
    println!("Hello, mlsolver ()!");

   let r1 = Tank {
       id : 1,
       elevation : 100.00,
       name :Some(String::from("Reservoir")), 
       head :100.0,
       pressure : Some(100.0),       
    };

    r1.print();

    let n2 = Junction {
        id : 2,
        elevation : 100.00,
        name :Some(String::from("n1")), 
        demand : 1.0,
        head :None,
        pressure : None,    
     };
 
     n2.print();

    let p1 = Pipe {
        id : 3,
        name :  Some(String::from("P3")),
        start : 0,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        velocity : None,
    };
       
    p1.print();

    let _pmp1= Pump {
        id: 2,
        name : Some(String::from("pmp1")),
        start : n2.id,
        end : r1.id,
        flow : None,
        head : None, 
        alpha : 10.0,
        beta : 20.0,
        gamma : 30.0,
    }; 

    _pmp1.print();

    let (a21, a10, q, r, h0) = network2();
    
    let chronos = Instant::now();
    
    let q_h = ml_solver(&a21, &a10, &q, &r, &h0);
    
    let duration = chronos.elapsed();

        match q_h {
        Some(results)=> { 
             print_vector(&results.0, "[Qi]:"); 
             print_vector(&results.1, "[Hi]:"); 
             println!("Iter = {}", results.2);
        },
        None => println!("fail!"),
    };
   
    println!("Computation time is : {:?}", duration);

    let mut vector = vec![0; 4];
    for q in 0..3 {
        vector.push(q);
    }

    println!("{:?}", q);

}

// network1() : return one (01) reservoir network
fn network1()-> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {

    let nn =4; // 04 junctions
    let np = 7;
    let no = 1; // one fixed head


    let mut h0 = vec![0.0f64; no];
    h0[0]=100.0; // 01 reservoir         

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
    let mut a10 = vec![vec![0.0f64; no]; np];
    a10[0][0]=-1.0f64;
    a10[1][0]=-1.0f64;

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


    (a21, a10, q, r, h0) 
 
}


// network2() : return two (02) reservoir network
fn network2()-> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {

    let nn =4; // 04 junctions
    let np = 7;
    let no = 2; // two (02) fixed head


    let mut h0 = vec![0.0f64; no];
    h0[0]=100.0; // 1st reservoir         
    h0[1]=100.0; // 2nd reservoir        

    let mut a21 = vec![vec![0.0f64; np]; nn];
    
    a21[0][0] = 1.0f64;     
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
    let mut a10 = vec![vec![0.0f64; no]; np];
    a10[0][0]=-1.0f64;
    a10[1][1]=-1.0f64;

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

    (a21, a10, q, r, h0)  
}

