use std::fmt::Display;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::utils::{ko, ok};
use crate::{branch::create_branch, vcs::internal_pager};
use sqlite::{Connection, Error, State};
use tabled::{Table, Tabled};
#[derive(Tabled, Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct TodoItem {
    #[tabled(rename = "ID")]
    pub id: i64,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Description")]
    pub description: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "Assigned to")]
    pub assigned_to: String,
    #[tabled(rename = "Due date")]
    pub due_date: String,
}

pub fn create_github_issue(title: &str, body: &str) -> bool {
    Command::new("gh")
        .arg("issue")
        .arg("create")
        .arg("--title")
        .arg(title)
        .arg("--body")
        .arg(body)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(".")
        .spawn()
        .expect("missing gh cli binary")
        .wait()
        .expect("failed to wait")
        .success()
}
impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let in_time = if self.due_date == "No limit" {
            "No limit".to_string()
        } else {
            let due_date = chrono::NaiveDate::parse_from_str(&self.due_date, "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            let today = chrono::Local::now().date_naive();
            if due_date < today {
                let days = (today - due_date).num_days();
                if days == 1 {
                    "Overdue by 1 day".to_string()
                } else {
                    format!("Overdue by {} days", days)
                }
            } else {
                let days = (due_date - today).num_days();
                if days == 0 {
                    "Due today".to_string()
                } else if days == 1 {
                    "Due tomorrow".to_string()
                } else {
                    format!("Just in time by {days} days")
                }
            }
        };
        write!(
            f,
            "ID: {}, Title: {}, Status: {}, Assigned to: {}, Due date: {}, In time: {}",
            self.id, self.title, self.status, self.assigned_to, self.due_date, in_time
        )
    }
}

pub fn check_and_reset_todos(_conn: &Connection) -> Result<(), Error> {
    Ok(())
}

pub fn start_todo(conn: &Connection, id: i64) -> Result<(), Error> {
    let query = "UPDATE todos SET status = 'IN_PROGRESS' WHERE id = ?";
    let mut stmt = conn.prepare(query)?;
    stmt.bind((1, id))?;
    stmt.next()?;
    ok(format!("Task #{id} is now in progress").as_str());
    Ok(())
}
pub fn add_todo(
    conn: &Connection,
    title: &str,
    description: &str,
    assigned_to: &str,
    due_date: &str,
) -> Result<(), Error> {
    let query = "INSERT INTO todos (title, description, assigned_to, due_date) VALUES (?, ?, ?, ?)";
    let mut stmt = conn.prepare(query)?;
    stmt.bind((1, title))?;
    stmt.bind((2, description))?;
    stmt.bind((3, assigned_to))?;
    stmt.bind((4, due_date))?;
    stmt.next()?;
    if Path::new(".git").is_dir() && create_github_issue(title, description) {
        ok("github issue created");
        Ok(())
    } else {
        ko("failed to create github issue check if github-cli is installed");
        Ok(())
    }
}

pub fn list_todos(conn: &Connection) -> Result<(), Error> {
    // On récupère les colonnes, en gérant les NULL potentiels avec des valeurs par défaut
    let todos = todos(conn)?;
    if todos.is_empty() {
        ok("No pending tasks. You're all caught up!");
    } else {
        let t = Table::new(&todos);
        if internal_pager(t.to_string()).is_ok() {
            ok("bye");
        }
    }
    Ok(())
}

pub fn todos(conn: &Connection) -> Result<Vec<TodoItem>, Error> {
    // On récupère les colonnes, en gérant les NULL potentiels avec des valeurs par défaut
    let query = "SELECT id,title, description, status, IFNULL(assigned_to, 'None'), IFNULL(due_date, 'No limit') FROM todos WHERE status != 'DONE' ORDER BY due_date ASC";
    let mut stmt = conn.prepare(query)?;
    let mut todos: Vec<TodoItem> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        todos.push(TodoItem {
            id: stmt.read(0)?,
            title: stmt.read(1)?,
            description: stmt.read(2)?,
            status: stmt.read(3)?,
            assigned_to: stmt.read(4)?,
            due_date: stmt.read(5)?,
        });
    }
    Ok(todos)
}

pub fn create_branches_from_todos(conn: &Connection) -> Result<(), Error> {
    for todo in &todos(conn)? {
        let branch_name = todo.title.replace(" ", "-").replace("_", "-").to_string();
        create_branch(conn, branch_name.as_str()).expect("failed to create the branch");
    }
    Ok(())
}
pub fn create_branches_from_todo(conn: &Connection, id: i64) -> Result<(), Error> {
    let mut stmt = conn.prepare("SELECT id, title, description, status, IFNULL(assigned_to, 'None'), IFNULL(due_date, 'No limit') FROM todos WHERE id = ?")?;
    stmt.bind((1, id))?;
    let mut todos: Vec<TodoItem> = Vec::new();
    while let Ok(State::Row) = stmt.next() {
        todos.push(TodoItem {
            id: stmt.read(0)?,
            title: stmt.read(1)?,
            description: stmt.read(2)?,
            status: stmt.read(3)?,
            assigned_to: stmt.read(4)?,
            due_date: stmt.read(5)?,
        });
    }
    for todo in &todos {
        let branch_name = todo.title.replace(" ", "-").replace("_", "-").to_string();
        create_branch(conn, branch_name.as_str()).expect("failed to create the branch");
    }
    Ok(())
}

pub fn complete_todo(conn: &Connection, id: i64) -> Result<(), Error> {
    let query = "UPDATE todos SET status = 'DONE' WHERE id = ?";
    let mut stmt = conn.prepare(query)?;
    stmt.bind((1, id))?;
    stmt.next()?;
    ok(format!("Task #{id} done !").as_str());
    Ok(())
}
