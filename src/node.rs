//use super::*;

#[derive(Debug)]
pub enum NodeType {
    Junction,
    Reservoir, 
    Tank,
}

//------------------------------------Node-------------------------------

pub trait Node {

    fn node_type(&self)-> NodeType;
    
    fn print(&self){
        println!("{}", self.to_string());
    }

    fn to_string(&self)-> String;          
}