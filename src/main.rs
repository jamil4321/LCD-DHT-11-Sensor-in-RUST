#![no_std]
#![no_main]

use panic_itm;
use cortex_m;
use cortex_m_rt::entry;
use f3::hal::stm32f30x;
use f3::hal::prelude::*;
use f3::hal::i2c::I2c;
use f3::hal::stm32f30x::I2C1;
use f3::hal::delay::Delay;
// Import for convert u8 to String
use core::fmt::Write;
use heapless::consts::*;
use heapless::String;
// Import Lcd Crate
use i2c_Lcd_lib::LiquidCrystalI2c;
// Import DHT11 crate
use dht_sensor_rust::dht11;

#[entry]
fn main()->!{
    // Initialize Peripherals 
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    // Initialize Flash & RCC
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Initialize Clock
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Initialize GPIOB For I2C Protocol
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    // Initialize PIN For I2C Protocol
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    // Initialize Delay
    let mut delay = Delay::new(cp.SYST, clocks);

    // Initialize I2c Protocol for Channel 1
    I2c::i2c1(dp.I2C1, (scl,sda), 400.khz(), clocks, &mut rcc.apb1);

    // Initialize GPIOA For DHT11 
    let mut gpioa = dp.GPIOA.split(& mut rcc.ahb);
    // Initialize PA3 as Open Drain outPut
    let mut pa3 = gpioa.pa3.into_open_drain_output(&mut gpioa.moder,&mut gpioa.otyper);
    pa3.internal_pull_up(&mut gpioa.pupdr,true);

    unsafe{
        // Initialize I2C1
        let i2c1 = &mut *(I2C1::ptr() as *mut _);
        // Initialize LCD 
        let mut lcd = LiquidCrystalI2c::new(0x3F, 16, 2,&mut delay,&i2c1);
        // Call begin Function
        lcd.begin();
        // Enable Cursor
        lcd.cursor();
        // Blink the Cursor
        lcd.blinkon();


    
    loop{
        // Start form Line 1
        lcd.line1(2u8);
        // Send Data as byte ASCII Code 
        for byte in b"Waiting for"{
            // Delay 100 milisecond Not Necessary
            lcd.delay.delay_ms(100u32);
            // Call Write Function to Send Data to LCD
            lcd.write(u8::from(*byte));

        }
         // Start form Line 2
        lcd.line2(0u8);
        // Send Data as byte ASCII Code 
        for byte in b"response DHT 11"{
            // Delay 100 milisecond Not Necessary
            lcd.delay.delay_ms(100u32);
            // Call Write Function to Send Data to LCD
            lcd.write(u8::from(*byte));
        }

        // Delay for DHT11 Make Sure Sensor is Ready..
        lcd.delay.delay_ms(2000u32);
        // Get DATA From DHT11 Sensor
        let (hum,_, temp,_) = dht11(&mut lcd.delay, &mut pa3);

        // Varibale For Temperature 
        let mut tempString = String::<U32>::from("Temperature ");
        // Convert temp u8 value to String and push it to tempString Varible
        let _ = write!(tempString, "{}%", temp);
        // Varibale For Humidity
        let mut humString = String::<U32>::from("Humidity ");
        // Convert hum u8 value to String and push it to humString Varible
        let _ = write!(humString, "{}*", hum);

        // Clear The Display
        lcd.clear();

        // Start From Line 1
        lcd.line1(0u8);
        // Send TempString Value as byte ASCII Code 
        for byte in tempString.as_bytes(){
            // Delay 100 milisecond Not Necessary
            lcd.delay.delay_ms(100u32);
            // Call Write Function to Send Data to LCD
            lcd.write(u8::from(*byte));

        }
        // Start From Line 1
        lcd.line2(2u8);
        // Send TempString Value as byte ASCII Code 
        for byte in humString.as_bytes(){
            // Delay 100 milisecond Not Necessary
            lcd.delay.delay_ms(100u32);
            // Call Write Function to Send Data to LCD
            lcd.write(u8::from(*byte));
        }
        // Wait for 1500 mililsecond 
        lcd.delay.delay_ms(1500u32);
        // Clear The Display
        lcd.clear();
    }
}
}