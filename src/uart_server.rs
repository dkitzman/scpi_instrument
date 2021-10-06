use alloc::string::String;

pub fn uart_transmit_server(_: &mut usize) {
    let mut subscriber = fe_osi::ipc::Subscriber::new("uart_tx").unwrap();
    loop {
        if let Some(message) = subscriber.get_message() {
            let s = String::from_utf8_lossy(&message);
            //write!(serial, "{}", s).unwrap();
            for c in s.chars() {
                fe_osi::putc(c);
            }
        }
    }
}

pub fn uart_receive_server(_: &mut usize) {
    let _publisher = fe_osi::ipc::Publisher::new("uart_rx").unwrap();
}
