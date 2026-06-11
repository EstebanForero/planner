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

    #[error("add course error")]
    AddCourseError,

    #[error("search courses error")]
    SearchCoursesError,

    #[error("get course error")]
    GetCourseError,

    #[error("relink class error")]
    RelinkClassError,

    #[error("delete all classes error")]
    DeleteAllClassesError,
}
