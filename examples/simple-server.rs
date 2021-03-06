// FIXME: check unused_imports
#![allow(unused_imports)]
// FIXME: check unused_variables
#![allow(unused_variables)]

extern crate clap;
#[macro_use] extern crate failure;
extern crate libmodbus_rs;

use clap::{App, Arg, ArgMatches};
use failure::Error;
use libmodbus_rs::{Modbus, ModbusRTU, ModbusTCP, ModbusTCPPI};

#[derive(Debug, Eq, PartialEq)]
enum Backend {
    TCP,
    TCPPI,
    RTU,
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    let backend;
    let modbus: Modbus;

    match matches.value_of("backend").unwrap() {
        "tcp" => backend = Backend::TCP,
        "tcppi" => backend = Backend::TCPPI,
        "rtu" => backend = Backend::RTU,
        _ => unreachable!(), // because clap ensures that for us
    }

    match backend {
        Backend::RTU => {
            let mut _query = vec![0u8; Modbus::RTU_MAX_ADU_LENGTH as usize];
            let serial_interface = matches.value_of("serial_interface").unwrap_or("/dev/ttyUSB0");
            let mut _modbus = Modbus::new_rtu(&serial_interface, 115200, 'N', 8, 1)?;
        },
        Backend::TCP => {
            let mut _query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
            let mut modbus = Modbus::new_tcp("127.0.0.1", 1502)?;

            let mut socket = modbus.tcp_listen(1)?;
            modbus.tcp_accept(&mut socket)?;
        },
        Backend::TCPPI => {
            let mut _query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
            let mut modbus = Modbus::new_tcp_pi("::0", "1502")?;

            let mut socket = modbus.tcp_listen(1)?;
            modbus.tcp_accept(&mut socket)?;
        },
    }

    Ok(())
}



fn main() {
    let matches = App::new("simple-client")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Simple Modbus Client with support for the different contextes (rtu, tcp, tcppi)!")
        .author("Stefan Müller (zzeroo) <s.mueller@it.kls-glt.de>")
        .arg(Arg::with_name("backend")
            .help("which backend shoud be used")
            .long("backend")
            .short("b")
            .possible_values(&["rtu", "tcp", "tcppi"])
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("serial_interface")
            .help("which backend shoud be used")
            .long("serial_interface")
            .short("s")
            .takes_value(true)
            .required(false))
        .get_matches();

    if let Err(ref err) = run(&matches) {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
