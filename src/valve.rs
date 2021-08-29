
#[derive(Debug)]
pub struct Valve{
    id : usize,
    name : Option<String>,
    start : usize,
    end : usize,
    linktype : LinkType,
    length : f64,
    diameter : f64,
    c_hw : f64,
    flow : Option<f64>,
    velocity : Option<f64>,
}


impl Link for Valve {
    fn resistance(&self)->f64 {
        10.00
        
    }

    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, category: {:?} , {}--->{} : diametre: {}, length: {}, R: {}", self.id, self.name, self.linktype, self.start, self.end, self.diameter, self.length, self.resistance())
    }    
}