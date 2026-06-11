use axum::{extract::{Json, Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, patch, post}, Router};
use domain::{CreateBlock, CreateClass, CreateSchedule, DeleteAllClasses, DeleteClass, GroupPlanningRequest, RankedParametersEndpoint, RelinkClass};
use general_repository::postgres_db::PostgresPlannerRepository;
use planner_service::PlannerService;
use serde::Deserialize;
use tracing::error;

#[derive(Clone)]
struct AppState {
    repo: PostgresPlannerRepository,
}

pub fn planner_router(repo: PostgresPlannerRepository) -> Router {
    let state = AppState { repo };

    Router::new()
        // Schedules
        .route("/addSchedule", post(add_schedule))
        .route("/deleteSchedule/:schedule_id", delete(delete_schedule))

        // Classes
        .route("/addClass", post(create_class))
        .route("/deleteClass", delete(delete_class))
        .route("/getClass", post(get_class))
        .route("/getClasses/:user_id", get(get_classes))
        .route("/getClassesId/:user_id", get(get_classes_id))
        .route("/relinkClass", patch(relink_class))
        .route("/deleteAllClasses", delete(delete_all_classes))

        // Courses (shared catalog)
        .route("/searchCourses", get(search_courses))
        .route("/getCourse/:course_id", get(get_course))

        // Blocks
        .route("/addBlock", post(add_block))
        .route("/deleteBlock/:block_id", delete(delete_block))
        .route("/getBlocks/:schedule_id", get(get_blocks))

        // Planning
        .route("/planning/:user_id", get(generate_plannings))
        .route("/planningRanked", post(ranked_plannings))
        .route("/groupPlanning", post(group_planning))

        // User
        .route("/addUser", post(add_user))
        .with_state(state)
}

// schedules
async fn add_schedule(State(state): State<AppState>, Json(shedule): Json<CreateSchedule>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.create_schedule(shedule.course_id, shedule.schedule_name).await.map_err(|_| {
        error!("error in create schedule");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn delete_schedule(State(state): State<AppState>, Path(schedule_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.remove_schedule(schedule_id).await.map_err(|err| {
        error!("error in remove schedule: {}", err);

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

// class
async fn create_class(State(state): State<AppState>, Json(class): Json<CreateClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.create_class(class.user_id, class.class_name, class.course_id).await.map_err(|_| {
        error!("error in create_class");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn delete_class(State(state): State<AppState>, Json(class): Json<DeleteClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.remove_class(class.user_id, class.class_id).await.map_err(|_| {
        error!("error in remove_class");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn get_class(State(state): State<AppState>, Json(class): Json<DeleteClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let class = planner_service.obtain_class(class.user_id, class.class_id).await.map_err(|_| {
        error!("error in obtain_class");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let class = serde_json::to_string(&class).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], class)
}

async fn get_classes(State(state): State<AppState>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let classes = planner_service.obtain_classes(user_id).await.map_err(|_| {
        error!("error in obtain_classes");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let classes = serde_json::to_string(&classes).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], classes)
}

// Admin: wipes every class/schedule/course for all users. Restricted to user_id 1.
async fn delete_all_classes(State(state): State<AppState>, Json(req): Json<DeleteAllClasses>) -> impl IntoResponse {
    if req.user_id != 1 {
        return StatusCode::FORBIDDEN;
    }

    let planner_service = PlannerService::new(state.repo);

    planner_service.delete_all_classes().await.map_err(|_| {
        error!("error in delete_all_classes");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn relink_class(State(state): State<AppState>, Json(relink): Json<RelinkClass>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.relink_class(relink.user_id, relink.class_id, relink.course_id).await.map_err(|_| {
        error!("error in relink_class");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn get_classes_id(State(state): State<AppState>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let classes = planner_service.obtain_classes_id(user_id).await.map_err(|_| {
        error!("error in obtain_classes");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let classes = serde_json::to_string(&classes).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], classes)
}

// courses
#[derive(Deserialize)]
struct SearchCoursesQuery {
    q: String,
}

async fn search_courses(State(state): State<AppState>, Query(params): Query<SearchCoursesQuery>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let courses = planner_service.search_courses(params.q).await.map_err(|_| {
        error!("error in search_courses");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let courses = serde_json::to_string(&courses).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], courses)
}

async fn get_course(State(state): State<AppState>, Path(course_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let course = planner_service.obtain_course(course_id).await.map_err(|_| {
        error!("error in get_course");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let course = serde_json::to_string(&course).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], course)
}

// blocks
async fn add_block(State(state): State<AppState>, Json(block): Json<CreateBlock>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.add_block(block.schedule_id, block.block).await.map_err(|_| {
        error!("error in add_block");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn delete_block(State(state): State<AppState>, Path(block_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    planner_service.delete_block(block_id).await.map_err(|_| {
        error!("error in delete_block");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    StatusCode::OK
}

async fn get_blocks(State(state): State<AppState>, Path(schedule_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let blocks = planner_service.get_blocks(schedule_id).await.map_err(|_| {
        error!("error in delete_block");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let blocks = serde_json::to_string(&blocks).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], blocks)
}

// planning
async fn generate_plannings(State(state): State<AppState>, Path(user_id): Path<i32>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let planning = planner_service.generate_plannings(user_id).await.map_err(|_| {
        error!("error in generate_plannings");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let planning = serde_json::to_string(&planning).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], planning)
}

// planning
async fn ranked_plannings(State(state): State<AppState>, Json(ranking_parameters): Json<RankedParametersEndpoint>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let planning = planner_service.rank_plannings(ranking_parameters.user_id, ranking_parameters.ranked_parameters).await.map_err(|_| {
        error!("error in generate_plannings");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let planning = serde_json::to_string(&planning).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], planning)
}

// group planning
async fn group_planning(State(state): State<AppState>, Json(request): Json<GroupPlanningRequest>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let planning = planner_service.rank_group_plannings(request).await.map_err(|_| {
        error!("error in rank_group_plannings");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap();

    let planning = serde_json::to_string(&planning).unwrap();

    (StatusCode::OK, [("Content-Type", "application/json")], planning)
}

// user
async fn add_user(State(state): State<AppState>) -> impl IntoResponse {
    let planner_service = PlannerService::new(state.repo);

    let id = planner_service.create_user().await.map_err(|_| {
        error!("error in create_user()");

        StatusCode::INTERNAL_SERVER_ERROR
    }).unwrap(); 

    let id = serde_json::to_string(&id).unwrap();

    (StatusCode::OK, id)
}
