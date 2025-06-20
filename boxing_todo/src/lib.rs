mod err;
use std::error::Error;
use std::fs;
pub use err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        println!("debug 1");
        let parsed = json::parse(&fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e)
        })?).map_err(|e| ReadErr {
            child_err: Box::new(e)
        })?;
        println!("debug 2");
        let title: String = parsed["title"].to_string();
        let mut tasks = Vec::new();
        for task in parsed["tasks"].members(){
            tasks.push(Task {
                id: task["id"].to_string().parse::<u32>().map_err(|e| ReadErr {
            child_err: Box::new(e)
        })?,
                description: task["description"].to_string(),
                level: task["level"].to_string().parse::<u32>().map_err(|e| ReadErr {
            child_err: Box::new(e)
        })?
            })
        }
        if tasks.len() == 0 {
            return Err(Box::new(err::ParseErr::Empty));
        }
        let res = TodoList{
            title,
            tasks
        };
        return Ok(res)
    }
}

