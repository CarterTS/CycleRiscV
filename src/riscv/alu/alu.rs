use std::cell::RefCell;
use std::rc::Rc;

use super::super::Bus;
use super::{AluAdderModule, AluSubtractionModule, AluAndModule, AluOrModule, AluXorModule, AluSltiModule, AluSltiuModule, AluShiftLeftModule, AluShiftRightModule};

/// Arithmatic Logic Unit
pub struct ArithmaticLogicUnit
{
    src0: Rc<RefCell<Bus>>,
    src1: Rc<RefCell<Bus>>,
    output: Rc<RefCell<Bus>>,

    add: AluAdderModule,
    sub: AluSubtractionModule,
    and: AluAndModule,
    or: AluOrModule,
    xor: AluXorModule,
    slti: AluSltiModule,
    sltiu: AluSltiuModule,
    shiftleft: AluShiftLeftModule,
    shiftright: AluShiftRightModule,

    pub mode: usize,
    pub sub_flag: bool
}

impl ArithmaticLogicUnit
{
    /// Generate a new ArithmaticLogicUnit
    pub fn new(src0: Rc<RefCell<Bus>>, src1: Rc<RefCell<Bus>>, output: Rc<RefCell<Bus>>) -> Self
    {
        Self
        {
            add: AluAdderModule::new(src0.clone(), src1.clone(), output.clone()),
            sub: AluSubtractionModule::new(src0.clone(), src1.clone(), output.clone()),
            and: AluAndModule::new(src0.clone(), src1.clone(), output.clone()),
            or: AluOrModule::new(src0.clone(), src1.clone(), output.clone()),
            xor: AluXorModule::new(src0.clone(), src1.clone(), output.clone()),
            slti: AluSltiModule::new(src0.clone(), src1.clone(), output.clone()),
            sltiu: AluSltiuModule::new(src0.clone(), src1.clone(), output.clone()),
            shiftleft: AluShiftLeftModule::new(src0.clone(), src1.clone(), output.clone()),
            shiftright: AluShiftRightModule::new(src0.clone(), src1.clone(), output.clone()),

            src0,
            src1,
            output,

            mode: 0,
            sub_flag: false
        }
    }

    /// Tick the ALU
    pub fn tick(&self)
    {
        match self.mode
        {
            0b000 => if !self.sub_flag {self.add.tick()} else {self.sub.tick()},
            0b001 => self.shiftleft.tick(),
            0b010 => self.slti.tick(),
            0b011 => self.sltiu.tick(),
            0b100 => self.xor.tick(),
            0b101 => self.shiftright.tick(self.sub_flag),
            0b110 => self.or.tick(),
            0b111 => self.and.tick(),
            default => panic!("Bad ALU Mode: {:03b}", default)
        }
    }
}