
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
}

impl Pipe {

    fn headloss(&self)-> Option<f64> {

        let hl = match self.flow {
            Some(q) => Some(self.resistance()*q.powf(1.85)), 
            None => None,
        };
        hl
    }
}


impl Link for Pipe {
    fn link_type(&self)->LinkType {
        LinkType::Pipe
    }
    fn resistance(&self)->f64 {
        (10.67*self.length)/(self.c_hw.powf(1.852)*self.diameter.powf(4.8704))
    }

    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, category: {:?} , {}--->{} : diametre: {}, length: {}, R: {}", self.id, self.name, self.link_type(), self.start, self.end, self.diameter, self.length, self.resistance())
    }    
}