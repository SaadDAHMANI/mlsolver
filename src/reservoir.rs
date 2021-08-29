
//-----------------------------------Reservoir----------------------------

#[derive(Debug, Clone)]
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

