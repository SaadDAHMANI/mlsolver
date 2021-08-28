//use super::*;

#[derive(Debug)]
pub enum NodeType {
    Junction,
    Reservoir, 
    Tank,
}

#[derive(Debug)]
pub struct Node {
    id : usize,
    nodetype :NodeType,
    name : Option<String>,
    elevation : f64,
    head : Option<f64>,
    pressure :  Option<f64>,
}


impl Node {
    fn print(&self){
        println!("{}", self.to_string());
    }

    fn to_string(&self)-> String {
           format!("id: {}, name: {:?}, ategory: {:?}, elevation: {}, head: {:?}, pressure: {:?}", self.id, self.name, self.nodetype, self.elevation, self.head, self.pressure)        
    }
}