use super::super::reload_pipeline;
use crate::{
    cpu::interpreter::{common::StateSource, Engine},
    emu::Emu,
};

pub fn b(emu: &mut Emu<Engine>, instr: u16) {
    let branch_addr = reg!(emu.arm7, 15).wrapping_add(((instr as i32) << 21 >> 20) as u32);
    reg!(emu.arm7, 15) = branch_addr;
    reload_pipeline::<{ StateSource::Thumb }>(emu);
}

pub fn b_cond<const COND: u8>(emu: &mut Emu<Engine>, instr: u16) {
    if !emu.arm7.engine_data.regs.cpsr.satisfies_condition(COND) {
        return inc_r15!(emu.arm7, 2);
    }
    let branch_addr = reg!(emu.arm7, 15).wrapping_add((instr as i8 as i32 as u32) << 1);
    reg!(emu.arm7, 15) = branch_addr;
    reload_pipeline::<{ StateSource::Thumb }>(emu);
}

pub fn bx(emu: &mut Emu<Engine>, instr: u16) {
    let branch_addr = reg!(emu.arm7, instr >> 3 & 0xF);
    reg!(emu.arm7, 15) = branch_addr;
    reload_pipeline::<{ StateSource::R15Bit0 }>(emu);
}

pub fn bl_prefix(emu: &mut Emu<Engine>, instr: u16) {
    reg!(emu.arm7, 14) = reg!(emu.arm7, 15).wrapping_add(((instr as i32) << 21 >> 9) as u32);
    inc_r15!(emu.arm7, 2);
}

pub fn bl_suffix(emu: &mut Emu<Engine>, instr: u16) {
    let branch_addr = reg!(emu.arm7, 14).wrapping_add(((instr & 0x7FF) << 1) as u32);
    reg!(emu.arm7, 14) = reg!(emu.arm7, 15).wrapping_sub(1);
    reg!(emu.arm7, 15) = branch_addr;
    reload_pipeline::<{ StateSource::Thumb }>(emu);
}