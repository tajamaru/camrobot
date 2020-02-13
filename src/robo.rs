use rppal::gpio::{Gpio,OutputPin,Result};
use rppal::pwm::{Pwm,Channel,Polarity};

const GPIO_AI2: u8 = 24;
const GPIO_AI1: u8 = 23;
const GPIO_STBY: u8 = 25;
const GPIO_BI1: u8 = 26;
const GPIO_BI2: u8 = 13;

const GPIO_RIGHT_EYE: u8 = 27;
const GPIO_LEFT_EYE: u8 = 17;

#[derive(Debug, PartialEq)]
pub enum MoterSpeed{
    Stop,
    Slow,
    Middle,
    High,
}

trait OutPin{
    fn change_high(&mut self);
    fn change_low(&mut self);
}
impl OutPin for OutputPin{
    fn change_high(&mut self){
        if self.is_set_low() {
            self.set_high();
        }
    }
    fn change_low(&mut self){
        if self.is_set_high() {
            self.set_low();
        }
    }
}
#[derive(Debug)]
pub struct Moter {
    in_1: OutputPin,
    in_2: OutputPin,
    pwm: Pwm,
    speed: MoterSpeed,
}

impl Moter{
    pub fn new(gpio_in_1:u8,gpio_in_2:u8,channel:Channel) -> Result<Self>{
        let gpio = Gpio::new()?;
        let in_1 = gpio.get(gpio_in_1)?.into_output();
        let in_2 = gpio.get(gpio_in_2)?.into_output();
        let pwm = Pwm::with_frequency(channel, 2000.0, 0.0,Polarity::Normal , true).expect("pwm not work!");
        Ok(Moter {
            in_1,
            in_2,
            pwm,
            speed: MoterSpeed::Stop,
        })

    }

    fn speed_to_duty_cycle(&mut self, speed:&MoterSpeed)  {
        if self.speed == *speed {
            return
        }

        let duty = match speed {
            MoterSpeed::Slow => 0.3,
            MoterSpeed::Middle => 0.5,
            MoterSpeed::High => 0.7,
            MoterSpeed::Stop => 0.0,
        };
        self.pwm.set_duty_cycle(duty).expect("moter duty_cycle error");
    }

    /**
     *逆転
     * */
    fn ccw(&mut self, speed: MoterSpeed){
        self.in_1.change_high();
        self.in_2.change_low();
        self.speed_to_duty_cycle(&speed);
    }
    /**
     * 正転
     * */
    fn cw(&mut self, speed: MoterSpeed){
        self.in_1.change_low();
        self.in_2.change_high();
        self.speed_to_duty_cycle(&speed);
    }
    pub fn stop(&mut self){
        self.in_1.change_low();
        self.in_2.change_low();
        self.speed_to_duty_cycle(&MoterSpeed::Stop);
    }
}


#[derive(Debug)]
pub struct Robo{
    pub moter_right: Moter,
    pub moter_left: Moter,
    pub stby: OutputPin,
    pub left_eye: OutputPin,
    pub right_eye: OutputPin,
}
#[derive(Debug)]
pub enum Rolling{
    Normal,
    Reverse,
}

impl Robo{
    pub fn new() -> Result<Self>{
        let moter_left = Moter::new(GPIO_AI1,GPIO_AI2,Channel::Pwm0).expect("moter_1 not work");
        let moter_right = Moter::new(GPIO_BI1,GPIO_BI2,Channel::Pwm1).expect("moter_2 not work");
        let gpio = Gpio::new()?;
        Ok(Self{
            moter_right,
            moter_left,
            stby: gpio.get(GPIO_STBY)?.into_output(),
            left_eye: gpio.get(GPIO_LEFT_EYE)?.into_output(),
            right_eye: gpio.get(GPIO_RIGHT_EYE)?.into_output(),
         })
    }
//    pub fn new(moter_left:Moter,moter_right:Moter,stby_pin:u8) -> Result<Self>{
//        Ok(Self{
//            moter_right,
//            moter_left,
//            stby: Gpio::new()?.get(stby_pin)?.into_output(),
//        })
//    }
    pub fn ready(&mut self){
        if self.stby.is_set_low() {
            self.stby.set_high();
        }
    }
    pub fn stop(&mut self){
        if self.stby.is_set_high() {
            self.stby.set_low();
        }
        self.moter_left.stop();
        self.moter_right.stop();
    }

    pub fn move_left_crawler(&mut self, rolling:Rolling, speed:MoterSpeed) {
        if speed == MoterSpeed::Stop{
            self.moter_left.stop();
            return
        }  
        match rolling {
            Rolling::Normal => self.moter_left.cw(speed),
            Rolling::Reverse => self.moter_left.ccw(speed),
        }
    }
    pub fn move_right_crawler(&mut self, rolling:Rolling, speed:MoterSpeed) {
        if speed == MoterSpeed::Stop{
            self.moter_right.stop();
            return
        }  
        match rolling {
            Rolling::Normal => self.moter_right.cw(speed),
            Rolling::Reverse => self.moter_right.ccw(speed),
        }
    }
    pub fn eye_toggle(&mut self){
        self.left_eye.toggle();
        self.right_eye.toggle();
    }
}
