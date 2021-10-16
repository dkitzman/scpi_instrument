use crate::scpi_def::TREE;
use scpi::error::Result;
use scpi::prelude::{ArrayErrorQueue, ArrayVecFormatter, Context, Device};
use scpi::response::Formatter;

pub struct MyDevice;

pub fn scpi_parser(_: &mut usize) {
    let mut subscriber = fe_osi::ipc::Subscriber::new("stdin").unwrap();
    let mut publisher = fe_osi::ipc::Publisher::new("stdout").unwrap();

    let mut my_device = MyDevice {};

    let mut errors = ArrayErrorQueue::<10>::new();
    let mut context = Context::new(&mut my_device, &mut errors, TREE);

    //Response bytebuffer
    let mut buf = ArrayVecFormatter::<256>::new();

    loop {
        if let Some(message) = subscriber.get_message() {
            //Result
            let result = context.run(&message, &mut buf);

            if let Err(err) = result {
                let msg = err.get_message().to_vec();
                publisher.publish(msg).unwrap();
            } else {
                publisher.publish(buf.as_slice().to_vec()).unwrap();
            }
        }
    }
}
impl Device for MyDevice {
    fn cls(&mut self) -> Result<()> {
        Ok(())
    }

    fn rst(&mut self) -> Result<()> {
        Ok(())
    }
}
