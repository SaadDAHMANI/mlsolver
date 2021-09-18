
pub fn ml_solver_0(a21 : &Vec<Vec<f64>>, a10 : &Vec<Vec<f64>>,  h0:&Vec<f64>, q : &Vec<f64>, r : &Vec<f64>)->Option<(Vec<f64>, Vec<f64>, usize)> {

    let nn = a21.len();
    let np = a21[0].len();
    
    if nn<2 {return Option::None;}
    if np<1 {return Option::None;}
    
    let mut iter : usize =0;
    let itermax : usize = 40; 
    let objective_err : f64 =0.0001;
    
     let mut _a = vec![vec![0.0f64; np]; np]; //A
     let mut _b =vec![0.0f64; np]; // B
     let mut _c = vec![0.0f64; np];
     let mut _flowsq = vec![0.0f64; np];
     let mut _previous_q = vec![0.0f64; np];
     let mut _headsh =  vec![0.0f64; nn];
     let mut _previous_h =  vec![0.0f64; nn];
     
     let mut _coef_a = vec![0.0f64; np]; // ai
     let mut _coef_b = vec![0.0f64; np]; //bi
    
     let m : f64 = 10.0;
     let n : f64 = 2.0;
    
     let _a12 =transpose(a21);
    
     print(& a21, &"A21");
     //print(&_a12, &"A12");
     
     // step 0 : compute Qmax 
     let mut qmax : f64 =0.0;
     
    for i in 0..q.len() {
       qmax+=q[i];
    }
    // compute delta Q
    let deltaq = qmax/m;
    
    let mut stoploop : bool = false;
    
    while stoploop == false {
      // step 1- Update A & B 
    
      // initilize A & B if the first iteration
      if iter==0 {
        // step 1 : establish A & B using eq.19
        // 1.1- initilize A matrix :
         initilize_a(&mut _a, & r, qmax);
      
         //print(&_a,&"[A]");
        
        // 1.2- initiliza B matrix :
            
          //print_vector(&_b, &"[B]");      
         }
    
        else {
             
             //Updating A (eq13) & B (eq14):
    
             update_a_b(&mut _a, &mut _b, &_flowsq, &r, deltaq, n);
    
             //let mut _intpart : f64 =0.0;
            
            //for i in 0..np {
            //    _intpart=_flowsq[i]/deltaq;   
            //     _coef_a[i] = f64::trunc(_intpart)*deltaq;
            //     _coef_b[i] = f64::trunc(_intpart + f64::signum(_flowsq[i]))*deltaq;
            
            //      //Updating A (eq13):
            //     _intpart =(f64::powf(_coef_b[i], n)- f64::powf(_coef_a[i],n))/(_coef_b[i]-_coef_a[i]);
            //     _a[i][i]=f64::signum(_flowsq[i])*r[i]*_intpart;
                
            //      //Updating B (eq14):
            //     _b[i]=-1.0*f64::signum(_flowsq[i])*r[i]*(_intpart*_coef_a[i] - f64::powf(_coef_a[i],n));  
            //   }
                       
        }
    
         // Step 2 : Compute V (eq) and C 
         // Compute V:
         let inva = invers_diagonal(&_a);
         let inva = match inva {
                Ok(matrx)=>matrx,
                Err(error) => panic!("Problem with inverse diagonal matrix : {:?}", error),      
         };
    
         //print(&inva, "[A-]");
    
         let _v1 = product(&a21, &inva);
         let _v1 = match _v1{
             Ok(matrx)=> matrx,
             Err(error)=> panic!("Problem with product matrices : {:?}", error),
         };
    
         let _v = product(&_v1, &_a12);
         let _v = match _v {
             Ok(matrx)=>matrx,
             Err(error)=> panic!("Problem with product matrices : {:?}", error),
         };
    
         //print(&_v, "[V]");
    
         //Compute C:
          let _tmpc = product2(&a10, &h0);
          let tmpc = match _tmpc {
                Ok(vectr) => vectr,
                Err(error) =>  panic!("Problem with product matrix by vector : {:?}", error),
          };
    
         for i in 0..np {
             _c[i]=(-1.0*_b[i])- tmpc[i]; 
         }
    
         //print_vector(&_c, "C : ");
    
         // Step 3 : Compute H (eq.29)
         let invv = invers(&_v);
         
         let invv = match invv{
             Ok(matrix) => matrix,
             Err(error) => panic!("Problem with inverse matrix : {:?}", error),
            
         };
    
         let tmp = product2(&_v1, &_c);
    
         let mut tmp = match tmp {
             Ok(vectr) => vectr,
             Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
         };
    
         for i in 0..nn {
             tmp[i] -= q[i];   
         }
    
         let _h = product2(&invv, &tmp);
          _headsh = match _h {
             Ok(vect)=>vect,
             Err(error) =>  panic!("Problem with product matrix by vector : {:?}", error),
         };
    
         //print_vector(&_headsh, "[H] :");
    
         // Step 4 : Compute flowws Q (eq30)
          let tmpql = product2(&inva, &_c);
          let tmpql =match tmpql{
              Ok(vect)=>vect,
              Err(error) =>  panic!("Problem with product matrix by vector : {:?}", error),
          };
          
          let tmpqm = product(&inva, &_a12);
          let tmpqm =match tmpqm {
              Ok(matrx) => matrx,
              Err(error) => panic!("Problem with matrix multiplication : {:?}", error),
          };
    
          let tmpqr = product2(&tmpqm, &_headsh);
          let tmpqr = match tmpqr {
                Ok(vect) => vect,
                Err(error) => panic!("Problem with product matrix by vector : {:?}", error),        
          };
    
          for i in 0..np {
              _flowsq[i]=tmpql[i]-tmpqr[i];
          }
    
          //print_vector(&_flowsq, "[Q]");
    
    
          //
          print(&tmpqm, &String::from("A-1 x A12"));
    
          //Check convergence :
          
          stoploop = check_convergence(&_flowsq, &_previous_q, objective_err) & check_convergence(&_headsh, &_previous_h, objective_err); 
    
         //Copy data 
         for i in 0..np {
             _previous_q[i]=_flowsq[i];
         } 
    
         for j in 0..nn {
             _previous_h[j]=_headsh[j];
         }
    
          iter+=1;
          if iter >= itermax {stoploop=true;}
         }
          Some((_flowsq, _headsh, iter+1))
    }

    
fn initilize_a(result_a : &mut Vec<Vec<f64>>, pipe_resistance : &Vec<f64>, qmax : f64) {
    
    let np = pipe_resistance.len();
     if result_a.len() == np {
        if result_a[0].len()==np {
             for i in 0..np {
                 result_a[i][i]= pipe_resistance[i]*qmax;
             }
         }
     }
    }


fn update_a_b(a : &mut Vec<Vec<f64>>, b : &mut Vec<f64>, flowsq : &Vec<f64>, r : &Vec<f64>, deltaq : f64, n : f64) {

    let mut _intpart : f64 = 0.0;
    let mut _coef_a : f64 = 0.0;
    let mut _coef_b : f64 = 0.0;
    let np = a.len();
                     
    for i in 0..np {
         _intpart=flowsq[i]/deltaq;   
         _coef_a = f64::trunc(_intpart)*deltaq;
         _coef_b = f64::trunc(_intpart + f64::signum(flowsq[i]))*deltaq;
    
          //Updating A (eq13):
         _intpart =(f64::powf(_coef_b, n)- f64::powf(_coef_a, n))/(_coef_b - _coef_a);
         a[i][i]=f64::signum(flowsq[i])*r[i]*_intpart;
        
          //Updating B (eq14):
         b[i]=-1.0*f64::signum(flowsq[i])*r[i]*(_intpart*_coef_a - f64::powf(_coef_a,n));  
       } 
}
