#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::gpio::{Gpioa, Output, Pin, PushPull, U};

use core::{convert::Infallible, pin::pin, time::Duration};

use lilos::exec::sleep_for;
use stm32f3xx_hal::{pac as device, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut core = cortex_m::Peripherals::take().unwrap();
    let device = device::Peripherals::take().unwrap();

    let mut rcc = device.RCC.constrain();

    let mut gpioa = device.GPIOA.split(&mut rcc.ahb);
    let mut led = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    led.set_high().unwrap();

    let fut1 = pin!(blinky(Duration::from_millis(1000), &mut led));
    lilos::time::initialize_sys_tick(&mut core.SYST, 16_000_000);

    info!("Hello World!");
    lilos::exec::run_tasks(&mut [fut1], lilos::exec::ALL_TASKS)
}

async fn blinky(
    interval: Duration,
    led: &mut Pin<Gpioa, U<5>, Output<PushPull>>,
) -> Infallible {
    loop {
        info!("high");
        led.set_high().unwrap();
        sleep_for(interval).await;

        info!("low");
        led.set_low().unwrap();
        sleep_for(interval).await;
    }
}
