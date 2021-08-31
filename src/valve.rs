
#[derive(Debug, Clone)]
pub struct Valve{
    id : usize,
    name : Option<String>,
    start : usize,
    end : usize,
    linktype : LinkType,
    diameter : f64,
    flow : Option<f64>,
    k_value : f64,
    valvetype : ValveType,
  
}

impl Valve {
    fn resistance(&self, flow :f64)->f64 {
        
        let rq : f64 = match self.valvetype {
            ValveType::CV => {
                    if flow > 0.0 {self.k_value }
                    else { 10.0f64.powi(15) }                    
            } 
            _ => self.k_value
        };
        rq
    }
}


impl Link for Valve {
     fn link_type(&self)-> LinkType {
        LinkType::Valve(self.valvetype)
    }

    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, type: {:?}, {:?}, From {}--->{} : diametre: {}", self.id, self.name, self.linktype, self.valvetype, self.start, self.end, self.diameter)
    }    
}