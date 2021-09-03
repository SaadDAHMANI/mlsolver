
// network1() : return one (01) reservoir network
fn network1()-> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {

    let nn =4; // 04 junctions
    let np = 7;
    let no = 1; // one fixed head


    let mut h0 = vec![0.0f64; no];
    h0[0]=100.0; // 01 reservoir         

    let mut a21 = vec![vec![0.0f64; np]; nn];
    
    a21[0][0] =1.0f64;     
    a21[0][2] =-1.0f64;
    a21[0][3] =-1.0f64;
    a21[0][4] =-1.0f64;

    a21[1][1] =1.0f64;
    a21[1][2] =1.0f64;
    a21[1][5] =-1.0f64;

    a21[2][3] =1.0f64;
    a21[2][6] =-1.0f64;

    a21[3][4] =1.0f64;
    a21[3][5] =1.0f64;
    a21[3][6] =1.0f64;   
    
    
    //let mut a01 = ml_matrix("-1 -1 0 0 0 0 0");
    let mut a10 = vec![vec![0.0f64; no]; np];
    a10[0][0]=-1.0f64;
    a10[1][0]=-1.0f64;

    //let mut q = ml_matrix("0.1 0.2 0.3 0.4");
    let mut q = vec![0.0f64; nn];
    q[0]=0.1;
    q[1]=0.2;
    q[2]=0.3;
    q[3]=0.4;

    //let mut r = ml_matrix("1.5625 50 100 12.5 75 200 100");

    let mut r = vec![0.0f64; np];
    r[0]=1.5625;
    r[1]=50.0;
    r[2]=100.0;
    r[3]=12.50;
    r[4]=75.0;
    r[5]=200.0;
    r[6]=100.0;

    (a21, a10, h0, q, r) 
 
}


// network2() : return two (02) reservoir network
fn network2()-> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>, Vec<f64>) {

    let nn =4; // 04 junctions
    let np = 7;
    let no = 2; // two (02) fixed head


    let mut h0 = vec![0.0f64; no];
    h0[0]=100.0; // 1st reservoir         
    h0[1]=100.0; // 2nd reservoir        

    let mut a21 = vec![vec![0.0f64; np]; nn];
    
    a21[0][0] = 1.0f64;     
    a21[0][2] =-1.0f64;
    a21[0][3] =-1.0f64;
    a21[0][4] =-1.0f64;

    a21[1][1] =1.0f64;
    a21[1][2] =1.0f64;
    a21[1][5] =-1.0f64;

    a21[2][3] =1.0f64;
    a21[2][6] =-1.0f64;

    a21[3][4] =1.0f64;
    a21[3][5] =1.0f64;
    a21[3][6] =1.0f64;   
    
    
    //let mut a01 = ml_matrix("-1 -1 0 0 0 0 0");
    let mut a10 = vec![vec![0.0f64; no]; np];
    a10[0][0]=-1.0f64;
    a10[1][1]=-1.0f64;

    //let mut q = ml_matrix("0.1 0.2 0.3 0.4");
    let mut q = vec![0.0f64; nn];
    q[0]=0.1;
    q[1]=0.2;
    q[2]=0.3;
    q[3]=0.4;

    //let mut r = ml_matrix("1.5625 50 100 12.5 75 200 100");

    let mut r = vec![0.0f64; np];
    r[0]=1.5625;
    r[1]=50.0;
    r[2]=100.0;
    r[3]=12.50;
    r[4]=75.0;
    r[5]=200.0;
    r[6]=100.0;

    (a21, a10, h0, q, r)  
}

fn network3()->Network {
    let t1 = Tank {
        id :0,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let j1 = Junction {
        id : 1,
        name : Some(String::from("J1")),
        elevation : 0.0,
        demand : 0.02,
        head : None,
        pressure : None,
    };

    let j2 = Junction {
        id : 2,
        name : Some(String::from("J2")),
        elevation : 0.0,
        demand : 0.01,
        head : None,
        pressure : None,
    };

    let p1 = Pipe {
        id : 1,
        name :  Some(String::from("P1")),
        start : 0,
        end : 1,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity : None,
    };
    
    let p2 = Pipe {
        id : 2,
        name :  Some(String::from("P2")),
        start : 1,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : true,
        //velocity : None,
    };
    
    let p3 = Pipe {
        id : 3,
        name :  Some(String::from("P3")),
        start : 0,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity :None,
    };
    
    let ts = vec![t1];
    let js = vec![j1,j2];
    let ps = vec![p1,p2,p3];
    let pms = vec![];
    let rs = vec![];
    let vlvs = vec![];


    let net3 = Network {
        junctions :js,
        pipes : ps,
        tanks : ts,
        reservoirs : rs,
        pumps : pms,
        valves : vlvs,
    };

    net3
}

fn network4()->Network {

    let t1 = Tank {
        id :0,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let t2 = Tank {
        id :3,
        name : Some (String::from("T2")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let j1 = Junction {
        id : 1,
        name : Some(String::from("J1")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let j2 = Junction {
        id : 2,
        name : Some(String::from("J2")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let p1 = Pipe {
        id : 1,
        name :  Some(String::from("P1")),
        start : 0,
        end : 1,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity : None,
    };
    
    let p2 = Pipe {
        id : 2,
        name :  Some(String::from("P2")),
        start : 1,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity : None,
    };
    
    let p3 = Pipe {
        id : 3,
        name :  Some(String::from("P3")),
        start : 0,
        end : 2,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity : None,
    };
    
    let pmp1 = Pump {
        id : 4,
        name : Some(String::from("Pump1")),
        start : 3,
        end : 2,
        flow : None,
        head : None, 
        alpha : 10.0,
        beta : 20.0,
        gamma : 30.0,
        state : LinkStatus::Open,
    };

    let ts = vec![t1, t2];
    let js = vec![j1, j2];
    let ps = vec![p1, p2, p3];
    let pms = vec![pmp1];
    let rs = vec![];
    let vlvs = vec![];

    let net4 = Network {
        junctions :js,
        pipes : ps,
        tanks : ts,
        reservoirs : rs,
        pumps : pms,
        valves : vlvs,
    };

     net4
}

fn network5()-> Network {
    let t1 = Tank {
        id :0,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let r1 = Reservoir {
        id : 3,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let j1 = Junction {
        id : 1,
        name : Some(String::from("J1")),
        elevation : 0.0,
        demand : 0.1,
        head : None,
        pressure : None,
    };

    let mut j2 = j1.clone();
    j2.id =2;

    let mut j4 = j1.clone();
    j4.id = 4;
    j4.demand = 0.3;

    let p1 = Pipe {
        id : 1,
        name :  Some(String::from("P1")),
        start : 0,
        end : 1,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity : None,
    };

    let mut p2 = p1.clone();
    p2.id = 2;
    p2.start =1;
    p2.end = 2;
    p2.state = LinkStatus::Open;

    let mut p3 = p1.clone();
    p3.id = 3;
    p3.start =0;
    p3.end = 2;
    p3.state = LinkStatus::Open;
   
    let mut p4 = p1.clone();
    p4.id = 4;
    p4.start = 4;
    p4.end = 2;
    p4.state = LinkStatus::Open;


    let pmp1 = Pump {
        id : 4,
        name : Some(String::from("Pump1")),
        start : 3,
        end : 4,
        flow : None,
        head : None, 
        alpha : 10.0,
        beta : 20.0,
        gamma : 30.0,
        state : LinkStatus::Open,
    };

    let ts = vec![t1];
    let js = vec![j1, j2, j4];
    let ps = vec![p1, p2, p3, p4];
    let pms = vec![pmp1];
    let rvs = vec![r1];
    let vlvs = vec![];

    let net5 = Network {
        junctions :js,
        pipes : ps,
        tanks : ts,
        reservoirs : rvs,
        pumps : pms,
        valves : vlvs,
    };

     net5
} 

//network with CV (Check Valve)
fn network6()-> Network {
    let t1 = Tank {
        id :0,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let r1 = Reservoir {
        id : 3,
        name : Some (String::from("T1")),
        elevation : 0.0,
        head : 100.0,
        pressure : None,

    };

    let j1 = Junction {
        id : 1,
        name : Some(String::from("J1")),
        elevation : 0.0,
        demand : 0.01,
        head : None,
        pressure : None,
    };

    let mut j2 = j1.clone();
    j2.id =2;

    let mut j4 = j1.clone();
    j4.id = 4;
    j4.demand = 0.01;


    let p1 = Pipe {
        id : 1,
        name :  Some(String::from("P1")),
        start : 0,
        end : 1,
        length : 100.0,
        diameter : 0.100,
        c_hw : 130.0,
        flow : None,
        state : LinkStatus::Open,
        check_valve : false,
        //velocity : None,
    };

    //let mut p2 = p1.clone();
    //p2.id = 2;
    //p2.start =1;
    //p2.end = 2;
    //p2.state = LinkStatus::Open;

    let mut p3 = p1.clone();
    p3.id = 3;
    p3.start =0;
    p3.end = 2;
    p3.state = LinkStatus::Open;
   
    let mut p4 = p1.clone();
    p4.id = 4;
    p4.start = 4;
    p4.end = 2;
    p4.state = LinkStatus::Open;

    let v1 = Valve {
        id : 5,
        name : Some(String::from("V1")),
        start : 1,
        end : 2,
        diameter : 0.100,
        flow : None,
        k_value : 1.0,
        valvetype : ValveType::FCV,
        status : LinkStatus::Open,         
    };  

    let pmp1 = Pump {
        id : 4,
        name : Some(String::from("Pump1")),
        start : 3,
        end : 4,
        flow : None,
        head : None, 
        alpha : 50.0,
        beta : 20.0,
        gamma : 30.0,
        state : LinkStatus::Open,
    };

    let ts = vec![t1];
    let js = vec![j1, j2, j4];
    let ps = vec![p1, p3, p4];
    let pms = vec![pmp1];
    let rvs = vec![r1];
    let vlvs = vec![v1];

    let net6 = Network {
        junctions :js,
        pipes : ps,
        tanks : ts,
        reservoirs : rvs,
        pumps : pms,
        valves : vlvs,
    };

     net6
} 


