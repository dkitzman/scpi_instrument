use scpi::error::Result;
use scpi::prelude::{Command, CommandTypeMeta, Context, ErrorCode, Node, ResponseUnit, Tokenizer};

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
        response.data("Hello world!".as_bytes()).finish()
    }
}

pub struct MeasureVoltageCommand {}
impl Command for MeasureVoltageCommand {
    qonly!();

    fn query(
        &self,
        _context: &mut Context,
        _args: &mut Tokenizer,
        response: &mut ResponseUnit,
    ) -> Result<()> {
        let voltage = 12;
        let msg = alloc::format!("{} V", voltage).into_bytes();
        response.data(msg.as_slice()).finish()
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
    // User defined Subsystems
    Node {
        name: b"MEASure",
        optional: false,
        handler: None,
        sub: &[Node {
            name: b"VOLTage",
            optional: false,
            handler: Some(&MeasureVoltageCommand {}),
            sub: &[],
        }]
    }
];
