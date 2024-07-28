use core::fmt;
use std::io;
use chrono::{NaiveDateTime, Utc};

pub enum Status {
    Completed,  // Completed task
    Failed,     // Tasks that failed to be completed (not working on it)
    InProgress, // Tasks that are in progress
    Todo,       // Tasks that haven't started yet
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Completed => write!(f, "Completed"),
            Status::Failed => write!(f, "Failed"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Todo => write!(f, "To-do"),
        }
    }
}


pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub creation_date: NaiveDateTime,
    pub status: Status,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        category: String,
        creation_date: Option<NaiveDateTime>,
        status: Status,
        id: u32,
    ) -> Result<Self, String> {

        if name.len() > 100 {
            return Err(String::from("Name of task too long (> 100 characters)."));
        }

        if description.len() > 500 {
            return Err(String::from("Description of task too long (> 500 characters)."));
        }

        if category.len() > 50 {
            return Err(String::from("Category of task too long (> 50 characters)."));
        }

        let creation_date = creation_date.unwrap_or_else(|| Utc::now().naive_utc());


        Ok(Self {
            id: id,
            name: name,
            description: description,
            category: category,
            creation_date: creation_date,
            status: status,
        }
    )
    }

    pub fn new_from_user() -> Result<Self, String>  {
        let mut id = String::new();
        let mut name = String::new();
        let mut description  = String::new();
        let mut category = String::new();

        println!("Enter Task ID: ");
        io::stdin().read_line(&mut id).expect("Error");
        let id: u32 = id.trim().parse().unwrap_or(0);

        if id == 0 {
            return Err(String::from("Wrong ID number"));
        }

        println!("Enter Task Name: ");
        io::stdin().read_line(&mut name).expect("Error");
        println!("Enter Task Description: ");
        io::stdin().read_line(&mut description).expect("Error");
        println!("Enter Task Category: ");
        io::stdin().read_line(&mut category).expect("Error");

        Self::new(name, description, category, Option::None, Status::Todo, id)
    }

    /// Prints information about task
    pub fn print(&self) {
        println!(
            "Task: {} | Category: {} | Description: {} | Date: {} | Status: {}",
            self.name, self.category, self.description, self.creation_date, self.status
        )
    }

}
