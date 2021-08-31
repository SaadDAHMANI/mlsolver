#[derive(Debug, Clone)]
pub struct Network {
    junctions : Vec<Junction>,
    pipes : Vec<Pipe>,
    tanks : Vec<Tank>,
    reservoirs : Vec<Reservoir>,
    pumps : Vec<Pump>,
    valves : Vec<Valve>, 
}

impl Network {

     pub fn get_network(&self)-> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {
        
        let nn = self.junctions.len();
        let nt = self.tanks.len();
        let nr = self.reservoirs.len();
        //let nr = match &self.reservoirs {
        //    Some(reservoirs) => reservoirs.len(),
        //    None => 0, 
        //};

        let no = nt + nr;

        let npip = self.pipes.len();
        let npmp = self.pumps.len();
        let nvlv = self.valves.len();

        let np = npip + npmp + nvlv;

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

               // Valves :
               for k in 0..nvlv {
                if self.valves[k].start == self.junctions[i].id {
                    _a21[i][k+npip+npmp]=-1.0;
                } 
                else if self.valves[k].end == self.junctions[i].id {
                 _a21[i][k+npip+npmp]= 1.0;  
                }
            }

        }

        //Matrix A10 
        let mut _a10 = vec![vec![0.0f64; no]; np];
       
        // Tanks
            for j in 0..nt {
                
                // Tanks -Pipes
                for i in 0..npip {
                    if self.pipes[i].start == self.tanks[j].id {
                        _a10[i][j] = -1.0;
                    }
                    else if self.pipes[i].end == self.tanks[j].id {
                        _a10[i][j] = 1.0;
                    }
                }

                // Tanks - Pumps
                for i in 0..npmp {
                    if self.pumps[i].start == self.tanks[j].id {
                        _a10[i+npip][j] = -1.0;
                    }
                    else if self.pumps[i].end == self.tanks[j].id {
                        _a10[i+npip][j] = 1.0;
                    }
                } 

                // Tanks - Valves 
                for i in 0..nvlv {
                    if self.valves[i].start == self.tanks[j].id {
                        _a10[i+npip+npmp][j] = -1.0;
                    }
                    else if self.valves[i].end == self.tanks[j].id {
                        _a10[i+npip+npmp][j] = 1.0;
                    }
                } 
            }
            
             // Reservoirs 

             for j in 0..nr {
                
                 // Reservoirs - Pipes
                for i in 0..npip {
                    if self.pipes[i].start == self.reservoirs[j].id {
                        _a10[i][j+nt] = -1.0;
                    }
                    else if self.pipes[i].end == self.reservoirs[j].id {
                        _a10[i][j+nt] = 1.0;
                    }
                }
                
                // Reservoirs - Pumps
                for i in 0..npmp {
                    if self.pumps[i].start == self.reservoirs[j].id {
                        _a10[i+npip][j+nt] = -1.0;
                    }
                    else if self.pumps[i].end == self.reservoirs[j].id {
                        _a10[i+npip][j+nt] = 1.0;
                    }
                } 

                // Reservoirs - Valves 
                for i in 0..nvlv {
                    if self.valves[i].start == self.reservoirs[j].id {
                        _a10[i+npip+npmp][j+nt] = -1.0;
                    }
                    else if self.valves[i].end == self.reservoirs[j].id {
                        _a10[i+npip+npmp][j+nt] = 1.0;
                    }
                } 
                
            }          
          

         //junction demands :
         //let mut _q = vec![0.0f64; nn];
         //for i in 0..nn  {
         //   _q[i]= self.junctions[i].demand;    
         //}

         let _q = self.get_demands();

        //H0 : reservoirs + tanks
        let mut _h0 = vec![0.0f64; no];
        for k in 0..nt{
            _h0[k] = self.tanks[k].head;
        }

        for k in 0..nr{
            _h0[k+nt] = self.reservoirs[k].head;
        }
       
        //resistance for pipes
        //let mut _r = vec![0.0f64; npip];

         //for j in 0..npip  {
         //    _r[j]= self.pipes[j].resistance();
         //}
         let _r = self.get_pipes_resistances();
       
        (_a21, _a10, _h0, _q, _r,)

     }

     pub fn get_pipes_resistances(&self)-> Vec<f64> {
          //resistance for pipes
        let mut _r = vec![0.0f64; self.pipes.len()];

        for j in 0..self.pipes.len() {
            _r[j]= self.pipes[j].resistance();
        }

        _r
     }


     pub fn get_demands(&self)->Vec<f64> {

        let mut _q = vec![0.0f64; self.junctions.len()];

        for i in 0..self.junctions.len(){
            _q[i]= self.junctions[i].demand;    
        }
        _q
     }

     pub fn get_max_demand(&self)->f64 {
         self.get_demands().iter().sum()
     }




}




//*************************************** TESTS ***************************
#[cfg(test)]
mod network_tests {
use super::*;

    #[test]
    fn get_network_1tank() {

     let (a21, a10, h0, q, r) = network3().get_network();
          
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

     let (a21, a10, h0, q, r) = network4().get_network();
          
     let exp_a21 = vec![[1.0, -1.0, 0.0, 0.0], [0.0, 1.0, 1.0, 1.0]];
     let exp_a10 = vec![[-1.0, 0.0], [0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]];
     let exp_h0 = vec![100.00, 100.0];
     let exp_q =vec![0.1, 0.1];
     let exp_r = vec![9628.116375044583, 9628.116375044583, 9628.116375044583];
    
     assert_eq!(a21, exp_a21);
     assert_eq!(a10, exp_a10); 
     assert_eq!(h0, exp_h0);
     assert_eq!(q, exp_q);
     assert_eq!(r, exp_r);
    }

}

