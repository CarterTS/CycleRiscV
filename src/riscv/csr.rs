/// Address of a CSR (For clearer addressing)
pub enum CsrAddresses
{
    Mtvec = 0x305
}

/// CSR handling code
pub struct CsrHandler
{
    data: [u32; 4096],
}

impl CsrHandler
{
    /// Generate a new CsrHandler object
    pub fn new() -> Self
    {
        Self
        {
            data: [0; 4096]
        }
    }

    /// Read a Csr
    pub fn read_csr(&mut self, addr: u32) -> u32
    {
        self.data[(addr as usize)]
    }

    /// Write a Csr
    pub fn write_csr(&mut self, addr: u32, data: u32)
    {
        self.data[(addr as usize)] = data;
    }
}