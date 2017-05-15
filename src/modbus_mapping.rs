use errors::*;
use libmodbus_sys;
use std::io::Error;


/// To handle the mapping of your Modbus data, you must use this struct
///
#[derive(Debug)]
pub struct ModbusMapping {
    pub modbus_mapping: *mut libmodbus_sys::modbus_mapping_t,
}

impl ModbusMapping {
    /// `new` - allocate four arrays of bits and registers
    ///
    /// # Return values
    ///
    /// The function returns a Result containing the new allocated structure if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `number_bits` - How many bits sould allocated
    /// * `number_input_bits` - How many bits sould allocated
    /// * `number_registers` - How many registers sould allocated
    /// * `number_input_registers` - How many input registers sould allocated
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();
    /// ```
    pub fn new(number_bits: i32, number_input_bits: i32, number_registers: i32, number_input_registers: i32)
               -> Result<ModbusMapping> {
        unsafe {
            let modbus_mapping = libmodbus_sys::modbus_mapping_new(number_bits,
                                                                   number_input_bits,
                                                                   number_registers,
                                                                   number_input_registers);
            if modbus_mapping.is_null() {
                bail!(Error::last_os_error())
            } else {
                Ok(ModbusMapping { modbus_mapping: modbus_mapping })
            }
        }
    }

    /// `mapping_free` - free a modbus_mapping_t structure
    ///
    /// The function shall free the four arrays of mb_mapping_t structure and finally the mb_mapping_t referenced by
    /// mb_mapping.
    ///
    /// **It should not nessesary to call these function. Because rusts drop trait handles that for you!**
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::ModbusMapping;
    /// let mut modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();
    ///
    /// modbus_mapping.free();
    /// ```
    pub fn free(&mut self) {
        unsafe {
            libmodbus_sys::modbus_mapping_free(self.modbus_mapping);
        }
    }
}

impl Drop for ModbusMapping {
    fn drop(&mut self) {
        self.free()
    }
}
