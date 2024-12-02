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
}
