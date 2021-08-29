#[derive(Debug)]
pub struct Network {
    junctions : Vec<Junction>,
    pipes : Vec<Pipe>,
    tanks : Vec<Tank>,
    reservoirs : Option <Vec<Reservoir>>,
    pumps : Vec<Pump>,
    //valves : Option<Vec<Valve>>, 
}

impl Network {
     pub fn get_network(&self)-> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {
        
        let nn = self.junctions.len();
        let no = self.tanks.len();

        let npip = self.pipes.len();
        let npmp = self.pumps.len();
        let np = npip+npmp;

        //Matrix A21 
        let mut _a21 = vec![vec![0.0f64; np]; nn];

        for i in 0..nn {
             
               // Pipes :
               for j in 0..npip {
                   if self.pipes[j].start == self.junctions[i].id {
                    _a21[i][j]= -1.0;
                   }
                   else if self.pipes[j].end == self.junctions[i].id {
                    _a21[i][j]= 1.0;
                   }                  
               }
               
               // Pumps :
               for k in 0..npmp {
                   if self.pumps[k].start == self.junctions[i].id {
                       _a21[i][k+npip]=-1.0;
                   } 
                   else if self.pumps[k].end == self.junctions[i].id {
                    _a21[i][k+npip]= 1.0;  
                   }
               } 
        }

        //Matrix A10 
        let mut _a10 = vec![vec![0.0f64; no]; np];

       
            for j in 0..no {
                
                for i in 0..npip {
                    if self.pipes[i].start == self.tanks[j].id {
                        _a10[i][j] = -1.0;
                    }
                    else if self.pipes[i].end == self.tanks[j].id {
                        _a10[i][j] = 1.0;
                    }
                }
                
                for i in 0..npmp {
                    if self.pumps[i].start == self.tanks[j].id {
                        _a10[i+npip][j] = -1.0;
                    }
                    else if self.pumps[i].end == self.tanks[j].id {
                        _a10[i+npip][j] = 1.0;
                    }
                } 
            }                    


        //junction demands :
        let mut _q = vec![0.0f64; nn];

        for i in 0..nn  {
            _q[i]= self.junctions[i].demand;    
        }

        //H0 : reservoirs + tanks
        let mut _h0 = vec![0.0f64; no];
        
        for k in 0..no{
            _h0[k] = self.tanks[k].head;
        }

        //resistance for pipes
        let mut _r = vec![0.0f64; npip];

         for j in 0..npip  {
             _r[j]= self.pipes[j].resistance();
         }
       
        (_a21, _a10, _h0, _q, _r,)

     }
}




//*************************************** TESTS ***************************
#[cfg(test)]
mod network_tests {
use super::*;

    #[test]
    fn get_network_test1() {
    let t1 = Tank {
        id :0,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let j1 = Junction {
        id : 1,
        name : Some(String::from("J1")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let j2 = Junction {
        id : 2,
        name : Some(String::from("J2")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let p1 = Pipe {
        id : 1,
        name :  Some(String::from("P1")),
        start : 0,
        end : 1,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        velocity : None,
    };
    
    let p2 = Pipe {
        id : 2,
        name :  Some(String::from("P2")),
        start : 1,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        velocity : None,
    };
    
    let p3 = Pipe {
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
    
    let ts = vec![t1];
    let js = vec![j1,j2];
    let ps = vec![p1,p2,p3];
    let pms = vec![];

    let net1 = Network {
        junctions :js,
        pipes : ps,
        tanks : ts,
        reservoirs : None,
        pumps : pms,
    };

     let (a21, a10, h0, q, r) = net1.get_network();
          
     let exp_a21 = vec![[1.0, -1.0, 0.0], [0.0, 1.0, 1.0]];
     let exp_a10 = vec![[-1.0], [0.0], [-1.0]];
     let exp_h0 = vec![100.00];
     let exp_q =vec![0.1, 0.1];
     let exp_r = vec![9628.116375044583, 9628.116375044583, 9628.116375044583];
    
     assert_eq!(a21, exp_a21);
     assert_eq!(a10, exp_a10); 
     assert_eq!(h0, exp_h0);
     assert_eq!(q, exp_q);
     assert_eq!(r, exp_r);
    }


    #[test]
    fn get_network_2tanks_1pump() {
    let t1 = Tank {
        id :0,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let t2 = Tank {
        id :3,
        name : Some (String::from("T2")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let j1 = Junction {
        id : 1,
        name : Some(String::from("J1")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let j2 = Junction {
        id : 2,
        name : Some(String::from("J2")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let p1 = Pipe {
        id : 1,
        name :  Some(String::from("P1")),
        start : 0,
        end : 1,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        velocity : None,
    };
    
    let p2 = Pipe {
        id : 2,
        name :  Some(String::from("P2")),
        start : 1,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        velocity : None,
    };
    
    let p3 = Pipe {
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
    

    let pmp1 = Pump {
        id : 4,
        name : Some(String::from("Pump1")),
        start : 3,
        end : 2,
        flow : None,
        head : None, 
        alpha : 10.0,
        beta : 20.0,
        gamma : 30.0,
    };


    let ts = vec![t1];
    let js = vec![j1,j2];
    let ps = vec![p1,p2,p3];
    let pms = vec![];

    let net1 = Network {
        junctions :js,
        pipes : ps,
        tanks : ts,
        reservoirs : None,
        pumps : pms,
    };

     let (a21, a10, h0, q, r) = net1.get_network();
          
     let exp_a21 = vec![[1.0, -1.0, 0.0], [0.0, 1.0, 1.0]];
     let exp_a10 = vec![[-1.0], [0.0], [-1.0]];
     let exp_h0 = vec![100.00];
     let exp_q =vec![0.1, 0.1];
     let exp_r = vec![9628.116375044583, 9628.116375044583, 9628.116375044583];
    
     assert_eq!(a21, exp_a21);
     assert_eq!(a10, exp_a10); 
     assert_eq!(h0, exp_h0);
     assert_eq!(q, exp_q);
     assert_eq!(r, exp_r);
    }




}

