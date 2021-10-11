mod add_reg_a;
mod call;
pub(crate) mod consts;
mod data;
mod decrement_u16;
mod increment_u16;
mod jump;
mod load_a_hl_minus;
mod load_a_hl_plus;
mod load_bcde_reg_a;
mod load_mem_8b_reg_a;
mod load_mem_c_reg_a;
mod load_reg_a_mem_8b;
mod load_reg_a_mem_c;
mod pop;
mod push;
mod rel_jump;
mod restart;
mod returns;
mod rotate_left;
mod sub_reg_a;

pub use add_reg_a::AddRegA;
pub use call::Call;
pub use decrement_u16::DecRegNN;
pub use increment_u16::IncRegNN;
pub use jump::Jump;
pub use load_a_hl_minus::LoadRegAHLM;
pub use load_a_hl_plus::LoadRegAHLP;
pub use load_bcde_reg_a::LoadBCDERegA;
pub use load_mem_8b_reg_a::LoadMem8bRegA;
pub use load_mem_c_reg_a::LoadMemCRegA;
pub use load_reg_a_mem_8b::LoadRegAMem8b;
pub use load_reg_a_mem_c::LoadRegAMemC;
pub use pop::Pop;
pub use push::Push;
pub use rel_jump::RelJump;
pub use restart::Restart;
pub use returns::Return;
pub use rotate_left::RotateLeft;
pub use sub_reg_a::SubRegA;
