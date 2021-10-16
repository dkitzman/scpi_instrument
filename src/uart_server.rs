use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;
use cortex_m_semihosting::hprint;
use embedded_hal::serial::{Read as SerialRead, Write as SerialWrite};
use fe_osi::semaphore::Semaphore;

static UART_RX_LOCK: Semaphore = Semaphore::new(0);

pub fn uart_transmit_server<T: SerialWrite<u8> + Write>(serial: &mut T) {
    let mut subscriber = fe_osi::ipc::Subscriber::new("uart_tx").unwrap();
    loop {
        if let Some(message) = subscriber.get_message() {
            let s = String::from_utf8_lossy(&message);
            hprint!("{}", s).unwrap();
        }
    }
}

pub fn uart_receive_server<T: SerialRead<u8>>(serial: &mut T) {
    let mut publisher = fe_osi::ipc::Publisher::new("uart_rx").unwrap();

    loop {
        // block on getting rx data
        UART_RX_LOCK.take();
        let mut v = Vec::new();
        loop {
            match serial.read() {
                Ok(c) => {
                    v.push(c);
                }
                Err(_) => {
                    break;
                }
            };
        }
        if !v.is_empty() {
            publisher.publish(v).unwrap();
        }
        fe_osi::sleep(1);
    }
}

pub unsafe extern "C" fn uart_rx_isr() {
    UART_RX_LOCK.give();
}
