
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
