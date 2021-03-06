
#[derive(Debug, Clone)]
pub struct Pipe {
    id : usize,
    name : Option<String>,
    start : usize,
    end : usize,
    length : f64,
    diameter : f64,
    c_hw : f64,
    flow : Option<f64>,
    //velocity : Option<f64>,
    state : LinkStatus,
    check_valve : bool, 
}

impl Pipe {

    fn headloss(&self)-> Option<f64> {

        let hl = match self.flow {
            Some(q) => Some(self.resistance()*q.abs().powf(1.852)), 
            None => None,
        };
        hl
    }

    fn resistance(&self)->f64 {

        if self.state == LinkStatus::Open {
             (10.67*self.length)/(self.c_hw.powf(1.852)*self.diameter.powf(4.8704))
        }
        else {
             99.99f64.powi(20)
        }
       
    }

    fn get_r_of_q(&self, flow : f64)->f64 {

        if self.state == LinkStatus::Open {
            
            if self.check_valve { 
                 if flow < 0.0 {
                      99.99f64.powi(20)
                 }
                 else {
                    (10.675*self.length)/(self.c_hw.powf(1.852)*self.diameter.powf(4.8704))
                 }
             }

            else {
                (10.675*self.length)/(self.c_hw.powf(1.852)*self.diameter.powf(4.8704))
                }
           
        }
        else {
             99.99f64.powi(20)
        }       
    }


    fn velocity(&self)-> Option<f64> {
        let v = match self.flow {
            Some(q) => Some((4.0*q)/(std::f64::consts::PI*self.diameter.powi(2))), 
            None => None,
        };
        v
    }
}

impl Link for Pipe {
    fn link_type(&self)->LinkType {
        LinkType::Pipe
    }
   
    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, category: {:?} , {}--->{} : diametre: {}, length: {}, R: {}", self.id, self.name, self.link_type(), self.start, self.end, self.diameter, self.length, self.resistance())
    }    
}