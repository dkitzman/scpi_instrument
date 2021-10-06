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
                //break;
            }
        }
    }
}

use scpi::error::Result;
use scpi::prelude::*;

//Default commands
use scpi::ieee488::commands::*;
use scpi::scpi::commands::*;
use scpi::{
    ieee488_cls,
    ieee488_ese,
    ieee488_esr,
    ieee488_idn,
    ieee488_opc,
    ieee488_rst,
    ieee488_sre,
    ieee488_stb,
    ieee488_tst,
    ieee488_wai,
    //Helpers
    qonly,
    scpi_status,
    scpi_system,
    scpi_tree,
};

pub struct MyDevice;

/// # `[:EXAMple]:HELLO:WORLD?`
/// Example "Hello world" query
///
/// Will return `Hello world` as string response data.
pub struct HelloWorldCommand {}
impl Command for HelloWorldCommand {
    qonly!();

    fn query(
        &self,
        _context: &mut Context,
        _args: &mut Tokenizer,
        response: &mut ResponseUnit,
    ) -> Result<()> {
        response.data(b"Hello world!" as &[u8]).finish()
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

pub const TREE: &Node = scpi_tree![
    // Create default IEEE488 mandated commands
    ieee488_cls!(),
    ieee488_ese!(),
    ieee488_esr!(),
    ieee488_idn!(b"BAD Robotics", b"QEMU Test Project", b"00000000", b"0.1"),
    ieee488_opc!(),
    ieee488_rst!(),
    ieee488_sre!(),
    ieee488_stb!(),
    ieee488_tst!(),
    ieee488_wai!(),
    // Create default SCPI mandated STATus subsystem
    scpi_status!(),
    // Create default SCPI mandated SYSTem subsystem
    scpi_system!(),
    Node {
        name: b"EXAMple",
        optional: true,
        handler: None,
        sub: &[Node {
            name: b"HELLO",
            optional: false,
            handler: None,
            sub: &[Node {
                name: b"WORLD",
                optional: true,
                handler: Some(&HelloWorldCommand {}),
                sub: &[],
            }],
        },],
    }
];
