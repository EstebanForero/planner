use thiserror::Error;
pub type Result<T> = core::result::Result<T, PlannerError>;

#[derive(Debug, Error)]
pub enum PlannerError {
    #[error("add schedule error")]
    AddScheduleError,

    #[error("remove schedule error")]
    RemoveScheduleError,

    #[error("add class error")]
    AddClassError,

    #[error("remove class error")]
    RemoveClassError,

    #[error("get class error")]
    GetClassError,

    #[error("get classes error")]
    GetClassesError,

    #[error("get classes error")]
    GetClassesIdError,

    #[error("add block error")]
    AddBlockError,

    #[error("delete block error")]
    DeleteBlockError,

    #[error("get blocks error")]
    GetBlocksError,

    #[error("add user error")]
    AddUserError,
}
