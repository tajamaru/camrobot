pub mod robo;
pub mod controller;

use crate::robo::*;
use crate::controller::*;
use rppal::gpio::{Gpio};

use std::thread;
use std::time::Duration;


const GPIO_RIGHT_EYE: u8 = 27;
const GPIO_LEFT_EYE: u8 = 17;

fn select_controller() -> Box<dyn Controller>{
    match ProCon::new() {
        Ok(controller) => Box::new(controller),
        _ => Box::new(Keybord)
    }
}
fn main()  {

    let gpio = Gpio::new().unwrap();
    let mut left_eye = gpio.get(GPIO_LEFT_EYE).unwrap().into_output();
    let mut right_eye = gpio.get(GPIO_RIGHT_EYE).unwrap().into_output();
    right_eye.set_high();
    left_eye.set_high();
    thread::sleep(Duration::from_secs(1));
    left_eye.set_low();
    right_eye.set_low();
    
    let mut controller = select_controller();
    
    let mut robo = Robo::new().expect("error");    
    robo.ready();
//    robo.stop();

    loop {
        let (left_rolling, left_speed) = controller.get_left_crawler();
        let (right_rolling, right_speed) = controller.get_right_crawler();
        println!("left {:?}:right {:?}",left_speed,right_speed);
        robo.move_left_crawler(left_rolling, left_speed);
        robo.move_right_crawler(right_rolling, right_speed);
        
   }

    robo.stop();

}
