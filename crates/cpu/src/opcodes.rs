mod load_hl_8b;
mod load_mem_8b_reg_a;
mod load_r_b8;
mod load_r_r;
mod load_reg_hl;
mod load_rr_b16;

pub use load_hl_8b::LoadHL8b;
pub use load_mem_8b_reg_a::LoadMem8bRegA;
pub use load_r_b8::LoadR8b;
pub use load_r_r::LoadR1R2;
pub use load_reg_hl::LoadRegHL;
pub use load_rr_b16::LoadRR16b;
