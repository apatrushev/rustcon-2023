#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rtic::app;
use rtic_monotonics::systick::*;
use stm32f3xx_hal::{
    gpio::{Output, PushPull, PA5},
    prelude::*,
};

#[app(device = stm32f3xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output<PushPull>>,
        state: bool,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();

        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 36_000_000, systick_mono_token);

        let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(36.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let mut led = gpioa
            .pa5
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        led.set_high().unwrap();

        info!("Hello World!");
        blink::spawn().ok();

        (Shared {}, Local { led, state: false })
    }

    #[task(local = [led, state])]
    async fn blink(cx: blink::Context) {
        loop {
            if *cx.local.state {
                info!("high");
                cx.local.led.set_high().unwrap();
                *cx.local.state = false;
            } else {
                info!("low");
                cx.local.led.set_low().unwrap();
                *cx.local.state = true;
            }
            Systick::delay(1000.millis()).await;
        }
    }
}
