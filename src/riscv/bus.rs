/// 32 bit Bus
#[derive(Debug, Clone, Copy)]
pub struct Bus
{
    internal: u32
}

impl Bus
{
    /// Generate a new bus
    pub fn new() -> Self
    {
        Self
        {
            internal: 0
        }
    }

    /// Enable a value on the bus
    pub fn enable_value(&mut self, value: u32)
    {
        self.internal = value;
    }

    /// Read a value from the bus
    pub fn read_value(&self) -> u32
    {
        self.internal
    }

    /// Disable a connection to the bus
    pub fn disable(&mut self)
    {
        self.internal = 0;
    }
}