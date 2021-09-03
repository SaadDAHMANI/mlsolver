
#[derive(Debug, Copy, Clone)]
pub enum LinkType {
    Pipe,
    Pump,
    Valve(ValveType),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LinkStatus {
    Open,
    Closed,
}


#[derive(Debug, Copy, Clone)]
pub enum ValveType {
    FCV,
    PBV,
    PRV,
    TCV,
    PSV,
    GPV,
}

pub trait Link {
    fn link_type(&self)-> LinkType; 
    //fn resistance(&self)->f64; 
    fn to_string(&self)-> String;
    fn print(&self) {
        println!("{}", self.to_string());
    }
}


