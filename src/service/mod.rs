use axum::http::StatusCode;
use chrono::{NaiveDate, Utc, Datelike};
use serde::{Serialize, Deserialize};
use log::{info, error};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

type Db = Arc<RwLock<HashMap<Uuid, Employee>>>;

#[derive(Debug, Deserialize)]
pub struct EmployeeData {
    first_name: String,
    last_name: String,
    year_of_birth: i32,
    month_of_birth: u32,
    day_of_birth: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Employee {
    id: Uuid,
    first_name: String,
    last_name: String,
    date_of_birth: NaiveDate,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserDetails {
    handle: String,
    first_name: String,
    last_name: String,
    date_of_birth: NaiveDate,
    diplomas: Vec<String>,
    onboarded: bool,
}

#[derive(Debug, Clone)]
pub struct Service {
    employee_db: Db,
}

impl Service {
    pub fn new() -> Box<Service> {
        let employee_db =  Db::default();

        Box::new(Service {
            employee_db,
        })
    }

    pub async fn add_employee(&self, create_employee: EmployeeData) -> Result<Employee, StatusCode> {
        info!("processing employee data");

        let date_of_birth = self.validate_birthday(
            create_employee.year_of_birth,
            create_employee.month_of_birth,
            create_employee.day_of_birth
        ).await?;

        
        info!{"storing user"};

        let employee = Employee {
            id: Uuid::new_v4(),
            first_name: create_employee.first_name,
            last_name: create_employee.last_name,
            date_of_birth: date_of_birth,

        };
    
        self.employee_db.write().unwrap().insert(employee.id, employee.clone());
    
        return Ok(employee)
    }
    
    pub async fn list_employees(&self, offset: Option<usize>, limit: Option<usize>) -> Vec<Employee> {
        info!("getting list of employees");
    
        let employees = self.employee_db.read().unwrap();
    
        let employee_list = employees
            .values()
            .skip(offset.unwrap_or(0))
            .take(limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>();
    
        return employee_list;
    }
    
    pub async fn get_employee(&self, id: Uuid) -> Result<Employee, StatusCode> {
        info!("getting user details");

        let employee = self.employee_db
                            .read()
                            .unwrap()
                            .get(&id)
                            .cloned()
                            .ok_or(StatusCode::NOT_FOUND)?;
    
        Ok(employee)
    }

    async fn validate_birthday(&self, year: i32, month: u32, day: u32) -> Result<NaiveDate, StatusCode> {
        let date_option = NaiveDate::from_ymd_opt(year, month, day);

        match date_option {
            Some(date) => {
                info!("Entered birth date is valid. Checking if user is of legal age");
                
                let naive_date_time_now = Utc::now().naive_utc();

                let now_year = naive_date_time_now.year();
                let now_month = naive_date_time_now.month();
                let now_day = naive_date_time_now.day();

                let year_difference = now_year - year;

                if year_difference < 18 || (
                            year_difference == 18 && (now_month < month || (
                                now_month == month && now_day < day
                            ))){
                    error!("User is underaged");
                    return Err(StatusCode::BAD_REQUEST);
                }
                return Ok(date);
            },
            None => {
                error!("Invalide data data for year: {year:?}, month: {month:?} and day: {day:?}");
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }  
}
