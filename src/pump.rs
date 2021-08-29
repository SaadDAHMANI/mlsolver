
#[derive(Debug, Clone)]
pub struct Pump{
    id : usize,
    name : Option<String>,
    start : usize,
    end : usize,
    flow : Option<f64>,
    head : Option<f64>, 
    alpha : f64,
    beta : f64,
    gamma : f64,
}

impl Pump {
    fn head_of(&self, flow :f64)->f64 {
       return self.alpha*flow.powi(2)+ self.beta * flow + self.gamma; 
    }
}


impl Link for Pump {

    fn link_type(&self)->LinkType {
        LinkType::Pump
    }

    fn resistance(&self)->f64 {
       -1.0f64
    }

    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, category: {:?} , {}--->{}, Curve H-Q: {}Q^2 + {}Q + {}", 
        self.id, self.name, self.link_type(), self.start, self.end, self.alpha, self.beta, self.gamma)
    }    
}