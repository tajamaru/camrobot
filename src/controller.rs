use gilrs::{Gilrs, Axis,GamepadId,Event};
use crate::robo::{Rolling,MoterSpeed};


pub trait Controller {
    fn get_left_crawler(&mut self) -> (Rolling,MoterSpeed);
    fn get_right_crawler(&mut self) -> (Rolling,MoterSpeed);
}

pub struct ProCon{
    pub gilrs : Gilrs,
    gamepad_id : GamepadId
}

const PRO_CON_NAME : &'static str = "Nintendo Switch Pro Controller";
impl  ProCon {
    pub fn new() -> Result<Self,&'static str>{
        let err_msg = "Pro Con が認識できてません";
        Gilrs::new().map_err(|_| err_msg)
        .and_then(|gilrs|{
            if let Some((gamepad_id,_)) = gilrs.gamepads().find(|(_,pad)| pad.name() == PRO_CON_NAME){
                Ok(Self{
                    gilrs,
                    gamepad_id,
                })
            }else{
                Err(err_msg)
            }
        })
    }
    fn get_stick_y(&mut self, axis:Axis) -> (Rolling,MoterSpeed){
        self.gilrs.next_event();
        let gamepad = self.gilrs.gamepad(self.gamepad_id);
        let state = gamepad.state().value(gamepad.axis_code(axis).unwrap());
        let rolling = if state > 0.0 {
            Rolling::Normal
        } else {
            Rolling::Reverse
        };

        match state.abs()  {
             d if d < 0.1 => (rolling,MoterSpeed::Stop),
             d if d < 0.3 => (rolling,MoterSpeed::Slow),
             d if d < 0.5 => (rolling,MoterSpeed::Middle),
             d if d <  1.0 => (rolling,MoterSpeed::High), 
             _ => (rolling,MoterSpeed::Stop),
         }

    }
}
impl Controller for ProCon {
    fn get_left_crawler(&mut self) -> (Rolling,MoterSpeed){
        self.get_stick_y(Axis::LeftStickY)
    }
    fn get_right_crawler(&mut self) -> (Rolling,MoterSpeed){
        self.get_stick_y(Axis::RightStickY)
    }
    
}

pub struct Keybord;
impl Controller for Keybord{
    fn get_left_crawler(&mut self) -> (Rolling,MoterSpeed){
        (Rolling::Normal,MoterSpeed::Middle)
    }
    fn get_right_crawler(&mut self) -> (Rolling,MoterSpeed){
        (Rolling::Normal,MoterSpeed::Middle)
    }

}