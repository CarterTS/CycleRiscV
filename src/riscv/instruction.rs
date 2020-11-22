/// Extract some bit range from a u32
fn extract_bit_range(val: u32, bit_low: usize, size: usize) -> u32
{
    (val >> bit_low) & ((1<<size) - 1)
}

/// Sign extend a u32 with the bit at some value
pub fn sign_extend(val: u32, bit: usize) -> u32
{
    let mask: u32 = !((2<<bit) - 1);

    if ((val >> bit) & 1) == 1
    {
        val | mask
    }
    else
    {
        val
    }
}

/// Instruction
#[derive(Debug, Clone, Copy)]
pub struct Instruction
{
    pub opcode: u8,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
    pub funct3: u8,
    pub immediate: u32
}

impl Instruction
{
    /// Generate a new instruction from a u32 read in from memory
    pub fn new(inst: u32) -> Self
    {
        let opcode = extract_bit_range(inst, 0, 7) as u8;
        let rd = extract_bit_range(inst, 7, 5) as u8;
        let funct3 = extract_bit_range(inst, 12, 3) as u8;
        let rs1 = extract_bit_range(inst, 15, 5) as u8;
        let rs2 = extract_bit_range(inst, 20, 5) as u8;
        let funct7 = extract_bit_range(inst, 25, 7) as u8;
        let immediate =

        // R Format
        if opcode == 0b0110011
        {
            0
        }
        // I Format
        else if opcode == 0b0010011 || opcode == 0b1100111 || opcode == 0b0000011 || opcode == 0b1110011
        {
            sign_extend(rs2 as u32 | ((funct7 as u32) << 5), 11)
        }
        // Store Format
        else if opcode == 0b0100011
        {
            sign_extend(rd as u32 | ((funct7 as u32) << 5), 11)
        }
        // Branch Format
        else if opcode == 0b1100011
        {
            sign_extend(
                (extract_bit_range(inst, 7, 1) << 11) |
                (extract_bit_range(inst, 8, 4) << 1) |
                (extract_bit_range(inst, 25, 6) << 5) |
                (extract_bit_range(inst, 31, 1) << 12),
                12
            )
        }
        // U Format
        else if opcode == 0b0110111 || opcode == 0b0010111
        {
           extract_bit_range(inst, 12, 20) << 12
        }
        // J Format
        else if opcode == 0b1101111
        {
            sign_extend(
                (extract_bit_range(inst, 12, 8) << 12) |
                (extract_bit_range(inst, 20, 1) << 11) |
                (extract_bit_range(inst, 21, 10) << 1) |
                (extract_bit_range(inst, 31, 1) << 20),
                20
            )
        }
        else
        {
            panic!("Unknown opcode format {:07b}", opcode);
        };

        Self
        {
            opcode,
            rd,
            rs1,
            rs2,
            funct7,
            funct3,
            immediate
        }
    }
}