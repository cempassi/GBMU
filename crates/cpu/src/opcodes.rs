mod add_reg_a;
mod data;
mod load_hl_8b;
mod load_a_hl_plus;
mod load_r_b8;
mod load_r_r;
mod load_reg_hl;
mod load_rr_b16;
mod sub_reg_a;

pub use add_reg_a::AddRegA;
pub use data::Data;
pub use load_hl_8b::LoadHL8b;
pub use load_a_hl_plus::LoadRegARegHLP;
pub use load_r_b8::LoadR8b;
pub use load_r_r::LoadR1R2;
pub use load_reg_hl::LoadRegHL;
pub use load_rr_b16::LoadRR16b;
pub use sub_reg_a::SubRegA;
