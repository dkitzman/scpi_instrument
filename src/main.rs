#![no_std]
#![no_main]

extern crate alloc;
use alloc::boxed::Box;

use stm32f407g_disc as board;

use crate::board::{
    hal::prelude::*,
    hal::stm32,
    serial::{config::Config, Serial},
};

use cortex_m::peripheral::Peripherals;
mod scpi_def;
mod scpi_parser;
mod stdio;
mod uart_server;

fn test_task(_: &mut usize) {
    let mut stdin = fe_osi::ipc::Publisher::new("stdin").unwrap();
    loop {
        let msg = alloc::format!("*IDN?").into_bytes();
        stdin.publish(msg).unwrap();
        fe_osi::sleep(100);

        let msg = alloc::format!("measure:voltage?").into_bytes();
        stdin.publish(msg).unwrap();
        fe_osi::sleep(100);
    }
}

#[no_mangle]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let mut cp = Peripherals::take().unwrap();
    fe_rtos::arch::arch_setup(&mut cp);

    let gpioa = p.GPIOA.split();

    // Constrain clock registers
    let rcc = p.RCC.constrain();

    // Configure clock to 168 MHz (i.e. the maximum) and freeze it
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    // USART2 at PA2 (TX) and PA3(RX) are connected to ST-Link
    // (well, not really, you're supposed to wire them yourself!)
    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();

    // Set up USART 2 configured pins and a baudrate of 115200 baud
    let serial = Serial::usart2(
        p.USART2,
        (tx, rx),
        Config::default().baudrate(115_200.bps()),
        clocks,
    )
    .unwrap();

    // Separate out the sender and receiver of the serial port
    let (tx, rx) = serial.split();

    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, stdio::stdout, None);
    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, stdio::stdin, None);
    fe_osi::task::task_spawn(
        fe_rtos::task::DEFAULT_STACK_SIZE,
        uart_server::uart_transmit_server,
        Some(Box::new(tx)),
    );

    fe_osi::task::task_spawn(
        fe_rtos::task::DEFAULT_STACK_SIZE,
        uart_server::uart_receive_server,
        Some(Box::new(rx)),
    );

    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, test_task, None);
    fe_osi::task::task_spawn(
        fe_rtos::task::DEFAULT_STACK_SIZE,
        scpi_parser::scpi_parser,
        None,
    );

    //Start the FeRTOS scheduler
    let enable_systick = |reload: usize| {
        cp.SYST.set_reload(reload as u32);
        cp.SYST.clear_current();
        cp.SYST.enable_counter();
        cp.SYST.enable_interrupt();
    };
    let reload_val = cortex_m::peripheral::SYST::get_ticks_per_10ms() / 10;
    fe_rtos::task::start_scheduler(enable_systick, reload_val as usize);

    panic!()
}
