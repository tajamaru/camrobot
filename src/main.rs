pub mod robo;
pub mod controller;

use crate::robo::*;
use crate::controller::*;




fn main()  {
    
    let  controller = &mut ProCon::new().expect("errrr");
    println!("controller ok!");
    let mut robo = Robo::new().expect("error");    
    println!("robo new ok!");
//    robo.stop();
    robo.wakeup(controller);


}
