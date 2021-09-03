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
include!("valve.rs");
include!("benchmark.rs");


fn main() {
    println!("Hello, mlsolver ()!");

    let network = network3();
    
    let chronos = Instant::now();
    
    let q_h = ml_solver(&network);
    
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
 
    
    
}

