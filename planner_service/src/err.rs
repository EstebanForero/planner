use thiserror::Error;
pub type Result<T> = core::result::Result<T, PlannerError>;

#[derive(Debug, Error)]
pub enum PlannerError {
    #[error("add schedule error")]
    AddScheduleError,
}
