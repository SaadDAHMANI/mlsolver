//use super::*;

#[derive(Debug)]
pub enum NodeType {
    Junction,
    Reservoir, 
    Tank,
}

//-----------------------------------Junction----------------------------
#[derive(Debug)]
pub struct Junction {
    id : usize,
    name : Option<String>,
    elevation : f64,
    head : Option<f64>,
    pressure :  Option<f64>,
}

impl Node for Junction {

    fn node_type(&self)-> NodeType{
        NodeType::Junction
    }
    fn to_string(&self)-> String {
           format!("id: {}, name: {:?}, ategory: {:?}, elevation: {}, head: {:?}, pressure: {:?}", self.id, self.name, self.node_type(), self.elevation, self.head, self.pressure)        
    }
}

//-----------------------------------Tank----------------------------

#[derive(Debug)]
pub struct Tank {
    id : usize,
    name : Option<String>,
    elevation : f64,
    head : Option<f64>,
    pressure :  Option<f64>,
}


impl Node for Tank {

    fn node_type(&self)-> NodeType{
        NodeType::Tank
    }
    fn to_string(&self)-> String {
           format!("id: {}, name: {:?}, ategory: {:?}, elevation: {}, head: {:?}, pressure: {:?}", self.id, self.name, self.node_type(), self.elevation, self.head, self.pressure)        
    }
}

//-----------------------------------Reservoir----------------------------

#[derive(Debug)]
pub struct Reservoir {
    id : usize,
    name : Option<String>,
    elevation : f64,
    head : Option<f64>,
    pressure :  Option<f64>,
}


impl Node for Reservoir {

    fn node_type(&self)-> NodeType{
        NodeType::Reservoir
    }
    fn to_string(&self)-> String {
           format!("id: {}, name: {:?}, ategory: {:?}, elevation: {}, head: {:?}, pressure: {:?}", self.id, self.name, self.node_type(), self.elevation, self.head, self.pressure)        
    }
}


//------------------------------------Node-------------------------------

pub trait Node {

    fn node_type(&self)-> NodeType;
    
    fn print(&self){
        println!("{}", self.to_string());
    }

    fn to_string(&self)-> String;          
}