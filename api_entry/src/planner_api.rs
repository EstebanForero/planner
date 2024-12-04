use axum::{extract::{Json, Path, State}, http::StatusCode, response::IntoResponse, Router};
use domain::{CreateBlock, CreateClass, CreateSchedule, DeleteClass};
use general_repository::postgres_db::PostgresPlannerRepository;
use planner_service::PlannerService;
use tracing::error;

#[derive(Clone)]
struct AppState {
    repo: PostgresPlannerRepository,
}

pub fn planner_router(repo: PostgresPlannerRepository) -> Router {
    let state = AppState { repo };

    Router::new()
        .with_state(state)
}

// schedules
async fn add_schedule(State(state): State<AppState>, Json(shedule): Json<CreateSchedule>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.create_schedule(shedule.class_id, shedule.schedule_name).await.map_err(|_| {
        error!("error in create schedule");
    }).unwrap();

    StatusCode::OK
}

async fn delete_schedule(State(state): State<AppState>, Path(schedule_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.remove_schedule(schedule_id).await.map_err(|_| {
        error!("error in remove schedule");
    }).unwrap();

    StatusCode::OK
}

// class
async fn create_class(State(state): State<AppState>, Json(class): Json<CreateClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.create_class(class.user_id, class.class_name).await.map_err(|_| {
        error!("error in create_class");
    }).unwrap();

    StatusCode::OK
}

async fn delete_class(State(state): State<AppState>, Json(class): Json<DeleteClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.remove_class(class.user_id, class.class_id).await.map_err(|_| {
        error!("error in remove_class");
    }).unwrap();

    StatusCode::OK
}

async fn get_class(State(state): State<AppState>, Json(class): Json<DeleteClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let class = planner_service.obtain_class(class.user_id, class.class_id).await.map_err(|_| {
        error!("error in obtain_class");
    }).unwrap();

    let class = serde_json::to_string(&class).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], class)
}

async fn get_classes(State(state): State<AppState>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let classes = planner_service.obtain_classes(user_id).await.map_err(|_| {
        error!("error in obtain_classes");
    }).unwrap();

    let classes = serde_json::to_string(&classes).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], classes)
}

// blocks
async fn add_block(State(state): State<AppState>, Json(block): Json<CreateBlock>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.add_block(block.schedule_id, block.block).await.map_err(|_| {
        error!("error in add_block");
    }).unwrap();

    StatusCode::OK
}

async fn delete_block(State(state): State<AppState>, Path(block_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.delete_block(block_id).await.map_err(|_| {
        error!("error in delete_block");
    }).unwrap();

    StatusCode::OK
}

async fn get_blocks(State(state): State<AppState>, Path(schedule_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let blocks = planner_service.get_blocks(schedule_id).await.map_err(|_| {
        error!("error in delete_block");
    }).unwrap();

    let blocks = serde_json::to_string(&blocks).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], blocks)
}

// planning
async fn generate_plannings(State(state): State<AppState>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let planning = planner_service.generate_plannings(user_id).await.map_err(|_| {
        error!("error in generate_plannings");
    }).unwrap();

    let planning = serde_json::to_string(&planning).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], planning)
}
