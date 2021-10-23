mod r#async;
mod calcul;
mod cb;
mod getat;
mod jump;
mod load;
mod nextpc;
mod set;
mod setat;

pub(crate) use calcul::Operation;
pub(crate) use cb::Operation as CbOperation;
pub(crate) use getat::GetAt;
pub(crate) use jump::Jump;
pub(crate) use nextpc::NextPc;
pub(crate) use r#async::Async;
pub(crate) use setat::SetAt;
