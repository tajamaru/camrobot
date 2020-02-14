pub mod robo;
pub mod controller;

use crate::robo::*;
use crate::controller::*;




fn main()  {
    
    let  controller = &mut ProCon::new().unwrap();
    
    let mut robo = Robo::new().expect("error");    
//    robo.stop();
    robo.wakeup(controller);


}
