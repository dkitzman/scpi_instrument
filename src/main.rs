#![no_std]
#![no_main]

extern crate alloc;

mod scpi_def;
mod stdio;
mod uart_server;

pub fn write_byte(c: u8) {
    let uart0: *mut usize = 0x4000_C000 as *mut usize;
    unsafe {
        *uart0 = c as usize;
    }
}

fn hello_task(_: &mut usize) {
    let mut stdout = fe_osi::ipc::Publisher::new("stdin").unwrap();
    loop {
        let msg = alloc::format!("HELLO:WORLD?").into_bytes();
        stdout.publish(msg).unwrap();
        // Give the subscribers enough time to clear the entries
        fe_osi::sleep(50);
    }
}

fn idn_task(_: &mut usize) {
    let mut stdin = fe_osi::ipc::Publisher::new("stdin").unwrap();
    loop {
        let msg = alloc::format!("*IDN?").into_bytes();
        stdin.publish(msg).unwrap();
        // Give the subscribers enough time to clear the entries
        fe_osi::sleep(50);
    }
}

#[no_mangle]
fn main() -> ! {
    let mut p = cortex_m::peripheral::Peripherals::take().unwrap();

    fe_rtos::arch::arch_setup(&mut p);

    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, stdio::stdout, None);
    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, stdio::stdin, None);
    fe_osi::task::task_spawn(
        fe_rtos::task::DEFAULT_STACK_SIZE,
        uart_server::uart_receive_server,
        None,
    );
    fe_osi::task::task_spawn(
        fe_rtos::task::DEFAULT_STACK_SIZE,
        uart_server::uart_transmit_server,
        None,
    );
    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, idn_task, None);
    fe_osi::task::task_spawn(fe_rtos::task::DEFAULT_STACK_SIZE, hello_task, None);
    fe_osi::task::task_spawn(
        fe_rtos::task::DEFAULT_STACK_SIZE,
        scpi_def::scpi_parser,
        None,
    );

    fe_osi::set_putc(|c: char| {
        write_byte(c as u8);
    });

    //Start the FeRTOS scheduler
    let enable_systick = |reload: usize| {
        p.SYST.set_reload(reload as u32);
        p.SYST.clear_current();
        p.SYST.enable_counter();
        p.SYST.enable_interrupt();
    };
    let reload_val = cortex_m::peripheral::SYST::get_ticks_per_10ms() / 10;
    fe_rtos::task::start_scheduler(enable_systick, reload_val as usize);

    loop {}
}
