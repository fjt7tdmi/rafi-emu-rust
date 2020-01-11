use core::*;

const PRIV_MACHINE: u32 = 3;

const CAUSE_EBREAK      : u32 = 3;
const CAUSE_ECALL_FROM_M: u32 = 11;

pub enum TrapType {
    Exception,
    TrapReturn,
}

pub struct Trap {
    pub trap_type: TrapType,
    pub cause: u32,
    pub value: u32,
    pub pc: u32,
}

impl Trap {
    pub fn new_ebreak(pc: u32) -> Trap {
        Trap { trap_type: TrapType::Exception, cause: CAUSE_EBREAK, value: 0, pc: pc }
    }

    pub fn new_ecall_from_machine(pc: u32) -> Trap {
        Trap { trap_type: TrapType::Exception, cause: CAUSE_ECALL_FROM_M, value: 0, pc: pc }
    }

    pub fn new_trap_return(pc: u32) -> Trap {
        Trap { trap_type: TrapType::TrapReturn, cause: 0, value: 0, pc: pc }
    }
}

fn process_exception(core: &mut Core, trap: &Trap)
{
    let mut mstatus = core.csr.read_mstatus();
    let mtvec = core.csr.read_mtvec();

    mstatus.set_mpie(mstatus.mie());
    mstatus.set_mie(0);
    mstatus.set_mpp(PRIV_MACHINE);

    core.csr.write_mstatus(mstatus);
    core.csr.write_mcause(trap.cause);
    core.csr.write_mepc(trap.pc);
    core.csr.write_mtval(trap.value);

    core.next_pc = mtvec.base();
}

fn process_trap_return(core: &mut Core, _trap: &Trap)
{
    let mut mstatus = core.csr.read_mstatus();
    let mepc = core.csr.read_mepc();

    mstatus.set_mpp(0);
    mstatus.set_mie(mstatus.mpie());

    core.csr.write_mstatus(mstatus);
    core.next_pc = mepc;
}

pub fn process_trap(core: &mut Core, trap: &Trap)
{
    match trap.trap_type {
        TrapType::TrapReturn => process_trap_return(core, trap),
        _ => process_exception(core, trap),
    }
}
