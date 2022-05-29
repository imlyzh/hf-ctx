use core::arch::asm;

use crate::abstract_model::{PartialContextOp, ContextOp};


#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Context {
    // pub save_params: bool,
    // pub save_float: bool,

    pub rip: u64,
    pub rflags: u64,

    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,

    pub r8: u64,
    pub r9: u64,

    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    // pub xmm0: u64,
    // pub xmm1: u64,
    // pub xmm2: u64,
    // pub xmm3: u64,
    // pub xmm4: u64,
    // pub xmm5: u64,
    // pub xmm6: u64,
    // pub xmm7: u64,
}

#[inline(always)]
pub fn get_rip() -> u64 {
    let r: u64;
    unsafe {
        // asm!("mov {}, [rsp]", out(reg) r);
        asm!("lea {}, [rip]", out(reg) r);
    }
    r
}

impl ContextOp for Context {
    #[inline(always)]
    fn save_context(&mut self) {
        unsafe {
            // asm!("mov {}, rflags", out(reg) self.rflags);

            asm!("mov {}, rax", out(reg) self.rax);
            asm!("mov {}, rbx", out(reg) self.rbx);
            asm!("mov {}, rcx", out(reg) self.rcx);
            asm!("mov {}, rdx", out(reg) self.rdx);
            asm!("mov {}, rsi", out(reg) self.rsi);
            asm!("mov {}, rdi", out(reg) self.rdi);
            asm!("mov {}, rbp", out(reg) self.rbp);
            asm!("mov {}, rsp", out(reg) self.rsp);
            asm!("mov {}, r8", out(reg) self.r8);
            asm!("mov {}, r9", out(reg) self.r9);
            asm!("mov {}, r10", out(reg) self.r10);
            asm!("mov {}, r11", out(reg) self.r11);
            asm!("mov {}, r12", out(reg) self.r12);
            asm!("mov {}, r13", out(reg) self.r13);
            asm!("mov {}, r14", out(reg) self.r14);
            asm!("mov {}, r15", out(reg) self.r15);

            // asm!("mov {}, xmm0", out(reg) self.xmm0);
            // asm!("mov {}, xmm1", out(reg) self.xmm1);
            // asm!("mov {}, xmm2", out(reg) self.xmm2);
            // asm!("mov {}, xmm3", out(reg) self.xmm3);
            // asm!("mov {}, xmm4", out(reg) self.xmm4);
            // asm!("mov {}, xmm5", out(reg) self.xmm5);
            // asm!("mov {}, xmm6", out(reg) self.xmm6);
            // asm!("mov {}, xmm7", out(reg) self.xmm7);

            self.rip = get_rip();
        }
    }

    #[inline(always)]
    fn apply_context(&mut self) {
        unsafe {
            asm!("mov {}, rax", in(reg) self.rax);
            asm!("mov {}, rbx", in(reg) self.rbx);
            asm!("mov {}, rcx", in(reg) self.rcx);
            asm!("mov {}, rdx", in(reg) self.rdx);
            asm!("mov {}, rsi", in(reg) self.rsi);
            asm!("mov {}, rdi", in(reg) self.rdi);
            asm!("mov {}, rbp", in(reg) self.rbp);
            asm!("mov {}, rsp", in(reg) self.rsp);
            asm!("mov {}, r8", in(reg) self.r8);
            asm!("mov {}, r9", in(reg) self.r9);
            asm!("mov {}, r10", in(reg) self.r10);
            asm!("mov {}, r11", in(reg) self.r11);
            asm!("mov {}, r12", in(reg) self.r12);
            asm!("mov {}, r13", in(reg) self.r13);
            asm!("mov {}, r14", in(reg) self.r14);
            asm!("mov {}, r15", in(reg) self.r15);

            // asm!("mov {}, xmm0", in(reg) self.xmm0);
            // asm!("mov {}, xmm1", in(reg) self.xmm1);
            // asm!("mov {}, xmm2", in(reg) self.xmm2);
            // asm!("mov {}, xmm3", in(reg) self.xmm3);
            // asm!("mov {}, xmm4", in(reg) self.xmm4);
            // asm!("mov {}, xmm5", in(reg) self.xmm5);
            // asm!("mov {}, xmm6", in(reg) self.xmm6);
            // asm!("mov {}, xmm7", in(reg) self.xmm7);

            // rflags

            asm!("jmp {}", in(reg) self.rip);
        }
    }

    #[inline(always)]
    fn return_from_trap(&mut self) {
        todo!()
    }
}

/*
impl PartialContextOp for Context {
    extern "C" fn save_par_ctx(&mut self) {
        if self.save_params {
            unsafe {
                asm!("mov {}, rdi", out(reg) (self.rdi));
                asm!("mov {}, rsi", out(reg) (self.rsi));
                asm!("mov {}, rdx", out(reg) (self.rdx));
                asm!("mov {}, rcx", out(reg) (self.rcx));
                asm!("mov {}, r8", out(reg) (self.r8));
                asm!("mov {}, r9", out(reg) (self.r9));
            }
        }
        if self.save_float {
            unsafe {
                asm!("movdqu {}, xmm0", out(reg) (self.xmm0));
                asm!("movdqu {}, xmm1", out(reg) (self.xmm1));
                asm!("movdqu {}, xmm2", out(reg) (self.xmm2));
                asm!("movdqu {}, xmm3", out(reg) (self.xmm3));
                asm!("movdqu {}, xmm4", out(reg) (self.xmm4));
                asm!("movdqu {}, xmm5", out(reg) (self.xmm5));
                asm!("movdqu {}, xmm6", out(reg) (self.xmm6));
                asm!("movdqu {}, xmm7", out(reg) (self.xmm7));
            }
        }

        unsafe {
            asm!("mov {}, r10", out(reg) (self.r10));
        }

    }

    extern "C" fn apply_par_ctx(&mut self) {

    }
}
 */
