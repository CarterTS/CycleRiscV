use std::cell::RefCell;

use super::Bus;

/// 32 bit register behaviors
pub trait Register32
{
    /// Get the value stored in the register (used for debug output, should
    /// not be used in the processor, all register io should occur via the
    /// bus connections)
    fn get_value(&self) -> u32;

    /// Enable a register on a bus
    fn enable_on_bus(&self, bus: &RefCell<Bus>);

    /// Disable the register on a bus
    fn disable_on_bus(&self, bus: &RefCell<Bus>);

    /// Set a register from a bus
    fn set_from_bus(&mut self, bus: &RefCell<Bus>);
}

/// Hardware zero 32 bit register
#[derive(Debug, Clone)]
pub struct HardwareZeroRegister
{

}

impl HardwareZeroRegister
{
    /// Generate a new hardware zero register
    pub fn new() -> Self
    {
        Self
        {
        }
    }
}

impl Register32 for HardwareZeroRegister
{
    fn get_value(&self) -> u32
    {
        0
    }

    fn enable_on_bus(&self, bus: &RefCell<Bus>)
    {
        bus.borrow_mut().enable_value(0);
    }

    fn disable_on_bus(&self, bus: &RefCell<Bus>)
    {
        bus.borrow_mut().disable();
    }

    fn set_from_bus(&mut self, bus: &RefCell<Bus>)
    {
        bus.borrow_mut().read_value();
    }
}

/// 32 bit register
#[derive(Debug, Clone)]
pub struct Register
{
    pub value: u32
}

impl Register
{
    /// Generate a new hardware zero register
    pub fn new() -> Self
    {
        Self
        {
            value: 0
        }
    }
}

impl Register32 for Register
{
    fn get_value(&self) -> u32
    {
        self.value
    }

    fn enable_on_bus(&self, bus: &RefCell<Bus>)
    {
        bus.borrow_mut().enable_value(self.value);
    }

    fn disable_on_bus(&self, bus: &RefCell<Bus>)
    {
        bus.borrow_mut().disable();
    }

    fn set_from_bus(&mut self, bus: &RefCell<Bus>)
    {
        self.value = bus.borrow_mut().read_value();
    }
}