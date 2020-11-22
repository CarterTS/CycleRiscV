/// Trait for memory access (by individual bytes)
pub trait MemoryAccess
{
    /// Read a byte from memory
    fn read_byte(&self, addr: u32) -> u8;

    /// Write a byte to memory
    fn write_byte(&mut self, addr: u32, data: u8);
}

/// Trait for memory access (by individual 16 bits at a time)
pub trait MemoryAccess16: MemoryAccess
{
    /// Read a half word from memory
    fn read_u16(&self, addr: u32) -> u16;

    /// Write a half word to memory
    fn write_u16(&mut self, addr: u32, data: u16);
}

/// Trait for memory access (by individual 32 bits at a time)
pub trait MemoryAccess32: MemoryAccess16
{
    /// Read a word from memory
    fn read_u32(&self, addr: u32) -> u32;

    /// Write a word to memory
    fn write_u32(&mut self, addr: u32, data: u32);
}

/// Ram Chip (512k)
pub struct Ram512k
{
    memory: [u8; 0x80000]
}

impl Ram512k
{
    /// Generate a new 512k ram chip
    pub fn new() -> Self
    {
        Self
        {
            memory: [0; 0x80000]
        }
    }
}

impl MemoryAccess for Ram512k
{
    fn read_byte(&self, addr: u32) -> u8
    {
        self.memory[(addr & 0x7FFFF) as usize]
    }

    fn write_byte(&mut self, addr: u32, data: u8)
    {
        self.memory[(addr & 0x7FFFF) as usize] = data;
    }
}

/// Ram Chip (1m)
pub struct Ram1m
{
    memory: [u8; 0x100000]
}

impl Ram1m
{
    /// Generate a new 1m ram chip
    pub fn new() -> Self
    {
        Self
        {
            memory: [0; 0x100000]
        }
    }
}

impl MemoryAccess for Ram1m
{
    fn read_byte(&self, addr: u32) -> u8
    {
        self.memory[(addr & 0xFFFFF) as usize]
    }

    fn write_byte(&mut self, addr: u32, data: u8)
    {
        self.memory[(addr & 0xFFFFF) as usize] = data;
    }
}

/// Rom Chip (1m)
pub struct Rom1m
{
    memory: [u8; 0x100000]
}

impl Rom1m
{
    /// Generate a new 1m rom chip
    pub fn new() -> Self
    {
        Self
        {
            memory: [0; 0x100000]
        }
    }
}

impl MemoryAccess for Rom1m
{
    fn read_byte(&self, addr: u32) -> u8
    {
        self.memory[(addr & 0xFFFFF) as usize]
    }

    fn write_byte(&mut self, _addr: u32, _data: u8)
    {
        
    }
}

/// Motherboard Memory Mapper
pub struct MotherboardMemory
{
    ram0: Ram512k,
    ram1: Ram512k
}

impl MotherboardMemory
{
    /// Generate a new MotherboardMemory
    pub fn new() -> Self
    {
        Self
        {
            ram0: Ram512k::new(),
            ram1: Ram512k::new()
        }
    }
}

impl MemoryAccess for MotherboardMemory
{
    fn read_byte(&self, addr: u32) -> u8
    {
        if addr & 0x80000 > 0
        {
            self.ram1.read_byte(addr)
        }
        else
        {
            self.ram0.read_byte(addr)
        }
    }

    fn write_byte(&mut self, addr: u32, data: u8)
    {
        if addr & 0x80000 > 0
        {
            self.ram1.write_byte(addr, data)
        }
        else
        {
            self.ram0.write_byte(addr, data)
        }
    }
}

impl MemoryAccess16 for MotherboardMemory
{
    fn read_u16(&self, addr: u32) -> u16
    {
        (self.read_byte(addr) as u16) | ((self.read_byte(addr.wrapping_add(1)) as u16) << 8)
    }

    fn write_u16(&mut self, addr: u32, data: u16)
    {
        self.write_byte(addr, (data & 0x000000FF) as u8);
        self.write_byte(addr.wrapping_add(1), ((data & 0x0000FF00) >> 8) as u8);
    }
}

impl MemoryAccess32 for MotherboardMemory
{
    fn read_u32(&self, addr: u32) -> u32
    {
        (self.read_byte(addr) as u32) | ((self.read_byte(addr.wrapping_add(1)) as u32) << 8) |
         ((self.read_byte(addr.wrapping_add(2)) as u32) << 16) |
         ((self.read_byte(addr.wrapping_add(3)) as u32) << 24)
    }

    fn write_u32(&mut self, addr: u32, data: u32)
    {
        self.write_byte(addr, (data & 0x000000FF) as u8);
        self.write_byte(addr.wrapping_add(1), ((data & 0x0000FF00) >> 8) as u8);
        self.write_byte(addr.wrapping_add(2), ((data & 0x00FF0000) >> 16) as u8);
        self.write_byte(addr.wrapping_add(3), ((data & 0xFF000000) >> 24) as u8);
    }
}