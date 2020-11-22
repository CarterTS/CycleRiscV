use std::cell::RefCell;
use std::rc::Rc;

use super::super::Bus;

/// Adder Module
pub struct AluAdderModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluAdderModule
{
    /// Generate a new AluAdderModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(s0.wrapping_add(s1));
    }
}

/// Subtraction Module
pub struct AluSubtractionModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluSubtractionModule
{
    /// Generate a new AluSubtractionModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(s0.wrapping_sub(s1));
    }
}

/// And Module
pub struct AluAndModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluAndModule
{
    /// Generate a new AluAndModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(s0 & s1);
    }
}

/// Or Module
pub struct AluOrModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluOrModule
{
    /// Generate a new AluOrModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(s0 | s1);
    }
}

/// Xor Module
pub struct AluXorModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluXorModule
{
    /// Generate a new AluXorModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(s0 ^ s1);
    }
}

/// Slti Module
pub struct AluSltiModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluSltiModule
{
    /// Generate a new AluSltiModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(if (s0 as i32) < (s1 as i32) {1} else {0});
    }
}

/// Sltiu Module
pub struct AluSltiuModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluSltiuModule
{
    /// Generate a new AluSltiuModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(if s0 < s1 {1} else {0});
    }
}

/// Shift Left Module
pub struct AluShiftLeftModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluShiftLeftModule
{
    /// Generate a new AluShiftLeftModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        self.output.borrow_mut().enable_value(s0 << (s1 & 0b11111));
    }
}

/// Shift Right Module
pub struct AluShiftRightModule
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>
}

impl AluShiftRightModule
{
    /// Generate a new AluShiftRightModule
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            src0,
            src1,
            output
        }
    }

    /// Tick the module
    pub fn tick(&self, flag: bool)
    {
        let s0 = self.src0.borrow().read_value();
        let s1 = self.src1.borrow().read_value();

        if flag
        {
            self.output.borrow_mut().enable_value(((s0 as i32) >> (s1 & 0b11111)) as u32);
        }
        else
        {
            self.output.borrow_mut().enable_value(s0 >> (s1 & 0b11111));
        }
    }
}