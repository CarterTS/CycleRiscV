use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use std::boxed::Box;

use super::Bus;
use super::Register32;

#[allow(unused_imports)]
use super::{MemoryAccess, MemoryAccess16, MemoryAccess32};
use super::MotherboardMemory;

use super::{HardwareZeroRegister, Register};

use super::ArithmaticLogicUnit;

use super::sign_extend;

use super::{CsrHandler, CsrAddresses};

/// Chip Mode (Keeps track of where in executing an instruction the processor pauses at)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChipMode
{
    LoadInstruction,
    ExecuteInstruction,
    LoadData,
    StoreResult,
    ExecuteJump,
    ExecuteBranch,
    BranchCheck,
}

/// RISCV 32I CPU Chip
pub struct ChipCPU
{
    src0_bus: Rc<RefCell<Bus>>,
    src1_bus: Rc<RefCell<Bus>>,
    alu_out_bus: Rc<RefCell<Bus>>,

    ram_addr_bus: Rc<RefCell<Bus>>,
    data: Rc<RefCell<Bus>>,

    clock: usize,

    inst: Register,
    pub program_counter: Register,
    output: Register,
    output2: Register,
    immediate: Register,

    mode: ChipMode,
    memory_mode: u8, // 0b00: Byte, 0b10: Half Word, 0b10: Word, oring 0b100 makes the result unsigned

    registers: [Box<dyn Register32>; 32],

    memory: Box<dyn MemoryAccess32>,

    alu: ArithmaticLogicUnit,

    pub debug_display: bool,

    csr_handle: CsrHandler
}

impl ChipCPU
{
    /// Generate a new ChipCPU object
    pub fn new() -> Self
    {
        let src0_bus = Rc::new(RefCell::new(Bus::new()));
        let src1_bus = Rc::new(RefCell::new(Bus::new()));
        let alu_out_bus = Rc::new(RefCell::new(Bus::new()));

        let ram_addr_bus = Rc::new(RefCell::new(Bus::new()));
        let data = Rc::new(RefCell::new(Bus::new()));

        Self
        {
            registers: [Box::new(HardwareZeroRegister::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),
                        Box::new(Register::new()),],
            
            alu: ArithmaticLogicUnit::new(src0_bus.clone(), src1_bus.clone(), alu_out_bus.clone()),

            src0_bus,
            src1_bus,
            alu_out_bus,

            ram_addr_bus,
            data,

            clock: 0,

            inst: Register::new(),
            program_counter: Register::new(),
            output: Register::new(),
            output2: Register::new(),
            immediate: Register::new(),

            mode: ChipMode::LoadInstruction,

            memory: Box::new(MotherboardMemory::new()),

            debug_display: false,

            memory_mode: 0,

            csr_handle: CsrHandler::new()
        }
    }

    /// Read a value from a register (should not be used baring for debug output)
    pub fn read_register_value(&self, reg: usize) -> u32
    {
        self.registers[reg].as_ref().get_value()
    }

    /// Set the memory to read
    pub fn memory_read(&mut self)
    {
        self.data.borrow_mut().enable_value(
            match self.memory_mode
            {
                0b000 => sign_extend(self.memory.read_byte(self.ram_addr_bus.borrow().read_value()) as u32, 7),
                0b001 => sign_extend(self.memory.read_u16(self.ram_addr_bus.borrow().read_value()) as u32, 15),
                0b010 => self.memory.read_u32(self.ram_addr_bus.borrow().read_value()),
                0b100 => self.memory.read_byte(self.ram_addr_bus.borrow().read_value()) as u32,
                0b101 => self.memory.read_u16(self.ram_addr_bus.borrow().read_value()) as u32,
                default => panic!("Unknown memory read mode {:03b}", default)
            });  
    }

    /// Set the memory to write
    pub fn memory_write(&mut self)
    {
        let addr = self.ram_addr_bus.borrow().read_value();
        let val = self.data.borrow().read_value();

        match self.memory_mode & 0b11
        {
            0b000 => self.memory.write_byte(addr, (val & 0xFF) as u8),
            0b001 => self.memory.write_u16(addr, (val & 0xFFFF) as u16),
            0b010 => self.memory.write_u32(addr, val),
            default => panic!("Unknown memory write mode {:03b}", default)
        }
    }

    /// Read from CSR
    pub fn csr_read(&mut self)
    {
        let addr = self.ram_addr_bus.borrow().read_value();
        self.data.borrow_mut().enable_value(self.csr_handle.read_csr(addr));
    }

    /// Write to CSR
    pub fn csr_write(&mut self)
    {
        let addr = self.ram_addr_bus.borrow().read_value();
        let val = self.data.borrow().read_value();
        
        self.csr_handle.write_csr(addr, val);
    }

    /// Clock to next instruction
    pub fn clock_to_instruction(&mut self)
    {
        self.clock_processor();

        while self.mode != ChipMode::LoadInstruction
        {
            self.clock_processor();
        }
    }

    /// Clock the processor
    pub fn clock_processor(&mut self)
    {
        self.alu.tick();

        match self.mode
        {
            ChipMode::LoadInstruction =>
            {
                // Read instruction from memory
                self.program_counter.enable_on_bus(&self.ram_addr_bus);
                self.memory_mode = 0b010;
                self.memory_read();
                self.inst.set_from_bus(&self.data);

                // Add 4 to the program counter
                self.program_counter.enable_on_bus(&self.src0_bus);
                self.src1_bus.borrow_mut().enable_value(4);
                self.alu.mode = 0;
                self.alu.sub_flag = false;
                self.alu.tick();
                self.output2.set_from_bus(&self.alu_out_bus);

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                self.mode = ChipMode::ExecuteInstruction;
            },
            ChipMode::ExecuteInstruction =>
            {
                let mut next_mode = ChipMode::StoreResult;

                // Decode the instruction (this would be done with discrete logic, this is mostly to generate the proper immediates)
                let instruction = super::Instruction::new(self.inst.get_value());

                self.immediate.value = instruction.immediate;
                // Arithmatic Immediate commands
                if instruction.opcode == 0b0010011
                {
                    self.registers[instruction.rs1 as usize].enable_on_bus(&self.src0_bus);
                    self.immediate.enable_on_bus(&self.src1_bus);
                    self.alu.mode = instruction.funct3 as usize;
                    self.alu.sub_flag = (instruction.funct7 & 0b0100000) > 0 && instruction.funct3 == 0b101;

                    self.alu.tick();
                }
                // Arithmatic Register Commands
                else if instruction.opcode == 0b0110011
                {
                    self.registers[instruction.rs1 as usize].enable_on_bus(&self.src0_bus);
                    self.registers[instruction.rs2 as usize].enable_on_bus(&self.src1_bus);
                    self.alu.mode = instruction.funct3 as usize;
                    self.alu.sub_flag = (instruction.funct7 & 0b0100000) > 0;

                    self.alu.tick();
                }
                // LUI
                else if instruction.opcode == 0b0110111
                {
                    self.immediate.enable_on_bus(&self.src0_bus);
                    self.registers[0].enable_on_bus(&self.src1_bus);

                    self.alu.mode = 0;
                    self.alu.sub_flag = false;

                    self.alu.tick();
                }
                // AUIPC
                else if instruction.opcode == 0b0010111
                {
                    self.immediate.enable_on_bus(&self.src0_bus);
                    self.program_counter.enable_on_bus(&self.src1_bus);

                    self.alu.mode = 0;
                    self.alu.sub_flag = false;

                    self.alu.tick();
                }
                // Load
                else if instruction.opcode == 0b0000011
                {
                    self.registers[instruction.rs1 as usize].enable_on_bus(&self.src0_bus);
                    self.immediate.enable_on_bus(&self.src1_bus);
                    self.alu.mode = 0;
                    self.alu.sub_flag = false;

                    self.alu.tick();

                    next_mode = ChipMode::LoadData;
                }
                // Store
                else if instruction.opcode == 0b0100011
                {
                    self.registers[instruction.rs1 as usize].enable_on_bus(&self.src0_bus);
                    self.immediate.enable_on_bus(&self.src1_bus);
                    self.alu.mode = 0;
                    self.alu.sub_flag = false;

                    self.alu.tick();

                    self.output.set_from_bus(&self.alu_out_bus);
                    self.output.enable_on_bus(&self.ram_addr_bus);

                    self.registers[instruction.rs2 as usize].enable_on_bus(&self.data);
                    self.memory_write();

                    next_mode = ChipMode::LoadInstruction;
                }
                // System Instruction
                else if instruction.opcode == 0b1110011
                {
                    // Is 'interrupt'
                    if instruction.funct3 == 0b000
                    {
                        // Make both act like 'ecall'
                        self.ram_addr_bus.borrow_mut().enable_value(CsrAddresses::Mtvec as u32);
                        self.csr_read();

                        self.program_counter.set_from_bus(&self.data);

                        next_mode = ChipMode::LoadInstruction;
                    }
                }
                // JAL
                else if instruction.opcode == 0b1101111
                {
                    // Add to the program counter
                    self.program_counter.enable_on_bus(&self.src0_bus);
                    self.immediate.enable_on_bus(&self.src1_bus);
                    self.alu.mode = 0;
                    self.alu.sub_flag = false;
                    self.alu.tick();

                    next_mode = ChipMode::ExecuteJump;
                }
                // JALR
                else if instruction.opcode == 0b1100111
                {
                    // Add to the program counter
                    self.immediate.enable_on_bus(&self.src0_bus);
                    self.registers[instruction.rs1 as usize].enable_on_bus(&self.src1_bus);
                    self.alu.mode = 0;
                    self.alu.sub_flag = false;
                    self.alu.tick();

                    next_mode = ChipMode::ExecuteJump;
                }
                // Branch
                else if instruction.opcode == 0b1100011 
                {
                    self.registers[instruction.rs1 as usize].enable_on_bus(&self.src0_bus);
                    self.registers[instruction.rs2 as usize].enable_on_bus(&self.src1_bus);
                    
                    // Eq
                    if instruction.funct3 & 0b110 == 0b000
                    {
                        self.alu.mode = 0;
                        self.alu.sub_flag = false;
                    }
                    // Lt
                    else if instruction.funct3 & 0b110 == 0b100
                    {
                        self.alu.mode = 0b010;
                        self.alu.sub_flag = false;
                    }
                    // Ltu
                    else if instruction.funct3 & 0b110 == 0b110
                    {
                        self.alu.mode = 0b011;
                        self.alu.sub_flag = false;
                    }

                    self.alu.tick();

                    self.output.set_from_bus(&self.data);

                    next_mode = ChipMode::BranchCheck;
                }
                
                if next_mode != ChipMode::BranchCheck && next_mode != ChipMode::ExecuteJump && next_mode != ChipMode::LoadInstruction
                {
                    // If the instruction is not a jump, then move the next address into the program counter
                    // Move value from OUT2 to PC
                    self.output2.enable_on_bus(&self.data);
                    self.program_counter.set_from_bus(&self.data);
                }

                self.output.set_from_bus(&self.alu_out_bus);

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                
                self.mode = next_mode;
            },
            ChipMode::LoadData =>
            {
                // Decode the instruction (this would be done with discrete logic, this is mostly to generate the proper immediates)
                let instruction = super::Instruction::new(self.inst.get_value());

                self.output.enable_on_bus(&self.ram_addr_bus);
                self.memory_mode = instruction.funct3;
                self.memory_read();
                self.registers[instruction.rd as usize].set_from_bus(&self.data);

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                self.mode = ChipMode::LoadInstruction;  
            },
            ChipMode::StoreResult =>
            {
                // Decode the instruction (this would be done with discrete logic, this is mostly to generate the proper immediates)
                let instruction = super::Instruction::new(self.inst.get_value());

                self.output.enable_on_bus(&self.data);
                self.registers[instruction.rd as usize].set_from_bus(&self.data);

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                self.mode = ChipMode::LoadInstruction;
            },
            ChipMode::ExecuteJump =>
            {
                // Decode the instruction (this would be done with discrete logic, this is mostly to generate the proper immediates)
                let instruction = super::Instruction::new(self.inst.get_value());

                self.output.enable_on_bus(&self.ram_addr_bus);
                self.program_counter.set_from_bus(&self.ram_addr_bus);

                self.output2.enable_on_bus(&self.data);
                self.registers[instruction.rd as usize].set_from_bus(&self.data);

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                self.mode = ChipMode::LoadInstruction;
            },
            ChipMode::BranchCheck =>
            {
                let mut next_mode = ChipMode::ExecuteBranch;

                // Decode the instruction (this would be done with discrete logic, this is mostly to generate the proper immediates)
                let instruction = super::Instruction::new(self.inst.get_value());

                let mut branch_cond =
                match instruction.funct3 & 0b110
                {
                    0b000 => self.output.value == 0,
                    0b100 | 0b110 => self.output.value == 1,
                    default => panic!("Unknown Funct3 for branch instruction {:03b}", default)
                };

                if instruction.funct3 & 0b001 > 0
                {
                    branch_cond = !branch_cond;
                }

                if branch_cond
                {
                    // Add to the program counter
                    self.program_counter.enable_on_bus(&self.src0_bus);
                    self.immediate.enable_on_bus(&self.src1_bus);
                    self.alu.mode = 0;
                    self.alu.sub_flag = false;
                    self.alu.tick();

                    self.output.set_from_bus(&self.alu_out_bus);
                }
                else
                {
                    self.output2.enable_on_bus(&self.data);
                    self.program_counter.set_from_bus(&self.data);

                    next_mode = ChipMode::LoadInstruction;
                }

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                self.mode = next_mode;
            },
            ChipMode::ExecuteBranch =>
            {
                self.output.enable_on_bus(&self.data);
                self.program_counter.set_from_bus(&self.data);

                if self.debug_display
                {
                    println!("{:?}", self);
                }

                self.mode = ChipMode::LoadInstruction;
            },
        }

        self.clock += 1;
    }

    /// Write data into a region of memory
    pub fn write_to_memory(&mut self, addr: u32, data: Vec<u8>)
    {
        for (i, val) in data.iter().enumerate()
        {
            self.memory.write_byte(addr.wrapping_add(i as u32), *val);
        }
    }
}

impl fmt::Debug for ChipCPU
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // CPU
        writeln!(f, "CPU")?;

        //  Registers:
        writeln!(f, " Registers:")?;

        //    PC: 0x00000000   OUT: 0x00000000
        writeln!(f, "   PC: 0x{:08X}   OUT: 0x{:08X}   OUT2: 0x{:08X}", self.program_counter.get_value(), self.output.get_value(), self.output2.get_value())?;

        //    R00: 0x00000000   R01: 0x00000000   R02: 0x00000000   R03: 0x00000000
        // ...
        // ...
        //    R28: 0x00000000   R29: 0x00000000   R30: 0x00000000   R31: 0x00000000
        for i in 0..8usize
        {
            for j in 0..4usize
            {
                write!(f, "   R{:02}: 0x{:08X}", 4 * i + j, self.read_register_value(4 * i + j))?;
            }

            writeln!(f, " ")?
        }

        //    INST: 0x00000000   0b00000000000000000000000000000000
        writeln!(f, "   INST: 0x{0:08X}   0b{0:032b}", self.inst.get_value())?;

        //  Buses:
        writeln!(f, " Buses:")?;

        //    RS0: 0x00000000   RS1: 0x00000000   RESULT: 0x00000000
        writeln!(f, "   SR0: 0x{:08X}   SR1: 0x{:08X}   OUT: 0x{:08X}", self.src0_bus.borrow().read_value(),
                                                                        self.src1_bus.borrow().read_value(),
                                                                        self.alu_out_bus.borrow().read_value())?;
        //    ADDR: 0x00000000   DATA: 0x00000000
        writeln!(f, "   ADDR: 0x{:08X}   DATA: 0x{:08X}", self.ram_addr_bus.borrow().read_value(), self.data.borrow().read_value())?;

        //  Misc:
        writeln!(f, " Misc:")?;

        //    Clk: 0000000000
        writeln!(f, "   Clk: {}", self.clock)?;

        //    Mode: LoadInstruction
        writeln!(f, "   Mode: {:?}", self.mode)?;

        //    Alu Mode: 0b000 (T)
        writeln!(f, "   Alu Mode: 0b{:03b}-{:01b} ({})", self.alu.mode, if self.alu.sub_flag {1} else {0},
            match self.alu.mode
            {
                0b000 => if self.alu.sub_flag {"-"} else {"+"},
                0b001 => "<<",
                0b010 => "<",
                0b011 => "<u",
                0b100 => "^",
                0b101 => if self.alu.sub_flag {">>u"} else {">>"},
                0b110 => "|",
                0b111 => "&",
                _ => "?"
            })?;

        //    Mem Mode: 0b000 (UB)
        writeln!(f, "   Mem Mode: 0b{:03b} ({}{})", self.memory_mode, 
            if self.memory_mode & 0b100 > 0 {"U"} else {""}, 
            match self.memory_mode & 0b11
            {
                0b00 => "B",
                0b01 => "H",
                0b10 => "W",
                0b11 => "D",
                _ => "E"
            })?;

        Ok(())
    }
}
