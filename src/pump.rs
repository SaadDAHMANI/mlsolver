
#[derive(Debug, Clone)]
pub struct Pump{
    id : usize,
    name : Option<String>,
    start : usize,
    end : usize,
    flow : Option<f64>,
    head : Option<f64>, 
    alpha : f64,
    beta : f64,
    gamma : f64,
    state : LinkStatus,

}

impl Pump {
    fn head_of(&self, flow :f64)->f64 {
       return self.alpha*flow.powi(2)+ self.beta * flow + self.gamma; 
    }

    fn head(&self)-> Option<f64> {
        let _hq = match self.flow {
            Some(q) => Some(self.alpha*q.powi(2)+ self.beta * q + self.gamma), 
            None => None,
        };
        _hq
     }

    fn get_rq(&self, flow : f64)->f64 {
        if self.state == LinkStatus::Open {
            self.alpha*flow + self.beta + (self.gamma/ flow) 
        } 
        else {
            10.00f64.powi(20)
        }       
    } 
}


impl Link for Pump {

    fn link_type(&self)->LinkType {
        LinkType::Pump
    }

    fn to_string(&self)-> String {
        format!("id: {}, name: {:?}, category: {:?} , {}--->{}, Curve H-Q: {}Q^2 + {}Q + {}", 
        self.id, self.name, self.link_type(), self.start, self.end, self.alpha, self.beta, self.gamma)
    }    
}

//************************************* TESTS **************************

#[cfg(test)]
mod pump_tests {
    use super::*;

    #[test]
    fn head_of_test1() {
        let pmp1 = Pump {
            id : 4,
            name : Some(String::from("Pump1")),
            start : 3,
            end : 2,
            flow : Some(0.017),
            head : None, 
            alpha : 10.0,
            beta : 20.0,
            gamma : 30.0,
            state : LinkStatus::Open,
        };

        let _h = pmp1.head_of(0.017);
         let exp_h = 30.34289;
         assert_eq!(_h, exp_h);
    }

    #[test]
    fn get_rq_when_pump_closed() {
        let pmp1 = Pump {
            id : 4,
            name : Some(String::from("Pump1")),
            start : 3,
            end : 2,
            flow : Some(0.017),
            head : None, 
            alpha : 10.0,
            beta : 20.0,
            gamma : 30.0,
            state : LinkStatus::Closed,
        };

        assert_eq!(pmp1.get_rq(0.01), 10.00f64.powi(20));
    }


}