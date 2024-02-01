use axum::{
    response::Html,
    extract::{Path, Query, State},
    routing::get,
    Router,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use log::{info, error};
use uuid::Uuid;

use crate::service::{Service, EmployeeData};

#[derive(Debug, Serialize, Default)]
struct CreatedUser {
    message: String,
}

// The query parameters for list onboarding requests
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub fn create_router() -> Router<> {
    let service = *Service::new();

    let app = Router::new()
                    .route("/", get(root))
                    .route("/user", get(list_employees).post(add_employee))
                    .route("/user/:id", get(get_employee))
                    .with_state(service);

    return app;
}

async fn root() -> Html<&'static str> {
    info!("serving root page");

    Html("<h1>Basic Web Service in Rust!</h1>")
}

async fn add_employee(State(service): State<Service>, Json(input): Json<EmployeeData>) -> Result<impl IntoResponse, StatusCode> {
    info!("adding new employee");

    let employee_result = service.add_employee(input).await;

    match employee_result {
        Ok(_user) => {
            info!{"User created succesfully"};
            let message = CreatedUser {
                message: String::from("User request created succesfully"),
            };

            return Ok(Json(message));
        },
        Err(_error) => {
            error!("Error creating user");

            return Err(StatusCode::BAD_REQUEST);
        }
        
    }
}

async fn list_employees(
    pagination: Option<Query<Pagination>>,
    State(service): State<Service>,
) -> impl IntoResponse {
    info!("getting list of employees");

    let Query(pagination) = pagination.unwrap_or_default();
    
    let employee_list = service.list_employees(pagination.offset, pagination.limit).await;

    info!("returning list of employees");

    Json(employee_list)
}

async fn get_employee(
    Path(id): Path<Uuid>,
    State(service): State<Service>
) -> Result<impl IntoResponse, StatusCode> {
    info!("get emloyee");
    
    let employee = service.get_employee(id).await?;

    info!("returning employee");

    Ok(Json(employee))
}
