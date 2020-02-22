use gilrs::{Gilrs, Axis,GamepadId,Event,EventType,Button};
use crate::robo::{Rolling,MoterSpeed,Action};


pub trait Controller {
    fn next_event(&mut self) -> Action;
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
    fn get_stick_y(&mut self, state:f32) -> (Rolling,MoterSpeed){
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
    fn next_event(&mut self) -> Action{
        let ev = self.gilrs.next_event().expect("error next event!"); 
        if ev.id != self.gamepad_id {
            return Action::None
        }
        match  ev {
            Event{  event:EventType::AxisChanged(Axis::LeftStickY,val,_),..} => {
                let (rolling,speed) = self.get_stick_y(val);
                Action::MoveLeftCrawler(rolling,speed)
            }
            Event{  event:EventType::AxisChanged(Axis::RightStickY,val,_),..} => {
                let (rolling,speed) = self.get_stick_y(val);
                Action::MoveRightCrawler(rolling,speed)
            }
            Event{   event: EventType::ButtonPressed(Button::RightTrigger, _),..} => {
                Action::ToggleEye
            },
            _ => Action::None,
        }
    }
    
}

