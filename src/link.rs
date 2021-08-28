
#[derive(Debug)]
pub enum LinkType {
    Pipe,
    Pump,
    Valve(ValveType),
}

#[derive(Debug)]
pub enum ValveType {
    FCV,
    PBV,
    PRV,
    TCV,
    PSV,
    GPV,
}

#[derive(Debug)]
pub struct Pipe{
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


impl Pipe {
    fn resistance(&self)->f64 {
        (10.67*self.length)/(self.c_hw.powf(1.852)*self.diameter.powf(4.8704))
    }

    fn print(&self) {
        println!("id: {}, name: {:?}, category: {:?} , {}--->{} : diametre: {}, length: {}, R: {}", self.id, self.name, self.linktype, self.start, self.end, self.diameter, self.length, self.resistance());
    }    
}


//pub trait Link {
//    fn resistance(&self)->f64 
//    fn print(&self)
//}


