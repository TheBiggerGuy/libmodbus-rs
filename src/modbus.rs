use raw::*;
use libc::{c_char, c_int};
use std::result;
use std::error;
use std::fmt;

// https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom
pub type Result<T> = result::Result<T, ModbusError>;


#[derive(Debug, Eq, PartialEq)]
pub enum ModbusError {
    /// Could not connect
    ConnectionError,
    /// Invalid modbus slave id.
    InvalidSlaveID,
    InvalidRTUSerialMode,
    InvalidRTURTS,
    InvalidDebug,
}

impl fmt::Display for ModbusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModbusError::ConnectionError => write!(f, "Could not connect."),
            ModbusError::InvalidSlaveID => write!(f, "Invalid modbus slave id."),
            ModbusError::InvalidRTUSerialMode => write!(f, "Invalid RTU serial mode."),
            ModbusError::InvalidRTURTS => write!(f, "Invalid RTU rts."),
            ModbusError::InvalidDebug => write!(f, "Invalid debug mode, only `true` of `false` are allowed."),
        }
    }
}

impl error::Error for ModbusError {
    fn description(&self) -> &str {
        match *self {
            ModbusError::ConnectionError => "Could not connect",
            ModbusError::InvalidSlaveID => "Invalid modbus slave id",
            ModbusError::InvalidRTUSerialMode => "Invalid RTU serial mode",
            ModbusError::InvalidRTURTS => "Invalid RTU rts",
            ModbusError::InvalidDebug => "Invalid debug mode",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ModbusError::ConnectionError => None,
            ModbusError::InvalidSlaveID => None,
            ModbusError::InvalidRTUSerialMode => None,
            ModbusError::InvalidRTURTS => None,
            ModbusError::InvalidDebug => None,
        }
    }
}

/// This struct holds the current context `ctx`
///
/// This logic is derived from that one libmodbus uses.
#[derive(Debug, Eq, PartialEq)]
pub struct Modbus { ctx: *mut modbus_t }

impl Modbus {
    /// Creates a new modbus context with the RTU backend
    ///
    /// # Attributes
    /// * `device`         - Device string e.g. "/dev/ttyUSB0"
    /// * `baud`           - Baud rate
    /// * `parity`         - Parity
    /// * `data_bit`       - Number of data bits
    /// * `stop_bit`       - Number of stop bits
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::Modbus;
    ///
    /// let modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// ```
    pub fn new_rtu(device: &str, baud: i32, parity: char, data_bit: i32, stop_bit: i32) -> Self {
        Modbus {
            ctx: {
                unsafe {
                    ::raw::modbus_new_rtu(::std::ffi::CString::new(device).unwrap().as_ptr(), baud, parity as c_char, data_bit, stop_bit)
                }
            }
        }
    }

    /// Define the slave ID of the remote device to talk in master mode or set the
    /// internal slave ID in slave mode
    ///
    /// # Attributes
    /// * `slave_id`    - New modbus slave id (valid range: `>= 0 && <= 247`), `0` is broadcast address
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `ModbusError::InvalidSlaveID` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus, ModbusError};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// assert_eq!(modbus.set_slave(10), Ok(0));
    /// assert_eq!(modbus.set_slave(255), Err(ModbusError::InvalidSlaveID));
    /// ```
    pub fn set_slave(&mut self, slave_id: i32) -> Result<i32> {
        unsafe {
            match ::raw::modbus_set_slave(self.ctx, slave_id) {
                -1 => Err(ModbusError::InvalidSlaveID),
                _ => Ok(0),
            }
        }
    }

    /// Set debug flag of the context
    ///
    /// # Attributes
    /// * `flag`    - boolean `true` or `false`
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `ModbusError::GenericError` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// assert_eq!(modbus.set_debug(true), Ok(0));
    /// ```
    pub fn set_debug(&mut self, flag: bool) -> Result<i32> {
        unsafe {
            match ::raw::modbus_set_debug(self.ctx, flag as c_int) {
                -1 => Err(ModbusError::InvalidDebug),
                _ => Ok(0),
            }
        }
    }

    /// Set the serial mode
    ///
    /// # Attributes
    /// * `mode`    - serial mode
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `ModbusError::GenericError` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// ```
    pub fn rtu_set_serial_mode(&mut self, mode: i32) -> Result<i32> {
        unsafe {
            match ::raw::modbus_rtu_set_serial_mode(self.ctx, mode) {
                -1 => Err(ModbusError::InvalidRTUSerialMode),
                _ => Ok(0),
            }
        }
    }

    /// Set the RTS mode in RTU
    ///
    /// # Attributes
    /// * `mode`    - serial mode
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `ModbusError::GenericError` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// assert_eq!(modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN), Ok(0));
    /// ```
    pub fn rtu_set_rts(&mut self, mode: i32) -> Result<i32> {
        unsafe {
            match ::raw::modbus_rtu_set_rts(self.ctx, mode) {
                -1 => Err(ModbusError::InvalidRTURTS),
                _ => Ok(0),
            }
        }
    }

    /// Establish a Modbus connection
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `ModbusError::GenericError` is returned
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// assert_eq!(modbus.connect(), Ok(0));
    /// ```
    pub fn connect(&self) -> Result<i32> {
        unsafe {
            match ::raw::modbus_connect(self.ctx) {
                -1 => Err(ModbusError::ConnectionError),
                _ => Ok(0),
            }
        }
    }

    /// modbus_close - close a Modbus connection
    ///
    /// The `modbus_close()` function shall close the connection established with the backend set in the context.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// match modbus.connect() {
    ///     Err(_) => { modbus.free(); }
    ///     Ok(_) => {
    ///         let _ = modbus.read_registers(0, 1);
    ///     }
    /// }
    /// modbus.close();
    /// modbus.free();
    /// ```
    pub fn close(&self) {
        unsafe {
            ::raw::modbus_close(self.ctx);
        }
    }

    /// Free a libmodbus context
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// modbus.free();
    /// ```
    pub fn free(&self) {
        unsafe {
            ::raw::modbus_free(self.ctx);
        }
    }

    /// Read many registers
    ///
    /// # Attributes
    /// * `address`    - Start address from which the read should start
    /// * `num_reg`    - Number of holding registers to read from `address` of the remote device
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN);
    /// let mut tab_reg: Vec<u16> = modbus.read_registers(0, 19);
    /// modbus.free();
    /// ```
    pub fn read_registers(&self, address: i32, num_reg: i32) -> Vec<u16> {
        let mut tab_reg = vec![0u16; 32];
        unsafe {
            ::raw::modbus_read_registers(self.ctx, address, num_reg, tab_reg.as_mut_ptr());
        }
        tab_reg
    }

    /// Read many registers
    ///
    /// # Attributes
    /// * `address`    - Start address from which the read should start
    /// * `num_reg`    - Number of holding registers to read from `address` of the remote device
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN);
    /// let mut tab_reg: Vec<u16> = modbus.read_input_registers(0, 19);
    /// modbus.free();
    /// ```
    pub fn read_input_registers(&self, address: i32, num_reg: i32) -> Vec<u16> {
        let mut tab_reg = vec![0u16; 32];
        unsafe {
            ::raw::modbus_read_input_registers(self.ctx, address, num_reg, tab_reg.as_mut_ptr());
        }
        tab_reg
    }

}



#[cfg(test)]
mod tests {
    use super::Modbus;

    #[test]
    fn crate_modbus() {
        let modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
        assert_eq!(modbus, modbus);
    }

}
