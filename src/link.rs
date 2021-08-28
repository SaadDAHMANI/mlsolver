
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

pub trait Link {
    fn resistance(&self)->f64; 
    fn to_string(&self)-> String;
    fn print(&self) {
        println!("{}", self.to_string());
    }
}


