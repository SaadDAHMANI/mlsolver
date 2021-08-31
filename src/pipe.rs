
#[derive(Debug, Clone)]
pub struct Pipe{
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

}

impl Pipe {

    fn headloss(&self)-> Option<f64> {

        let hl = match self.flow {
            Some(q) => Some(self.resistance()*q.powf(1.85)), 
            None => None,
        };
        hl
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
    fn resistance(&self)->f64 {

        if self.state == LinkStatus::Open {
            (10.67*self.length)/(self.c_hw.powf(1.852)*self.diameter.powf(4.8704))
        }
        else {
            10.00f64.powi(20)
        }
       
    }

    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, category: {:?} , {}--->{} : diametre: {}, length: {}, R: {}", self.id, self.name, self.link_type(), self.start, self.end, self.diameter, self.length, self.resistance())
    }    
}