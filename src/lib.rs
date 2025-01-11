use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    name: String,
    position: u8,
    quadrant: u8,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.position == other.position
            && self.quadrant == other.quadrant
    }
}

pub fn add_task(list: &mut Vec<Task>, new_task: String, quadrant: u8) {
    list.push(Task {
        name: new_task,
        position: (list.len() + 1) as u8,
        quadrant,
    });
}

fn organize_tasks(list: &mut Vec<Task>) {
    list.sort_by_key(|task| task.quadrant);
    list.iter_mut()
        .enumerate()
        .for_each(|(i, task)| task.position = (i + 1) as u8);
}

pub fn delete_task(list: &mut Vec<Task>, position: u8) {
    list.remove((position - 1) as usize);
    organize_tasks(list);
}

pub fn complete_task(list: &mut Vec<Task>) {
    list.remove(0);
    organize_tasks(list);
}

pub fn format_list(list: &Vec<Task>) -> String {
    list.iter()
        .map(|item| format!("{}. {}", item.position, item.name))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn save_task_list(list: &Vec<Task>) -> io::Result<()> {
    let file = File::create("todo_list.json")?;
    serde_json::to_writer(file, &list)?;
    Ok(())
}

pub fn load_task_list() -> io::Result<Vec<Task>> {
    let path = Path::new("todo_list.json");
    if path.exists() {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let task_list: Vec<Task> = serde_json::from_str(&content)?;

        Ok(task_list)
    } else {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, String::from("workout"), 1);
        assert_eq!(
            tasks,
            vec![Task {
                name: String::from("workout"),
                position: 1,
                quadrant: 1,
            }]
        );
    }

    #[test]
    fn test_display_list() {
        let tasks: Vec<Task> = vec![
            Task {
                name: String::from("workout"),
                position: 1,
                quadrant: 1,
            },
            Task {
                name: String::from("poop"),
                position: 2,
                quadrant: 2,
            },
            Task {
                name: String::from("code"),
                position: 3,
                quadrant: 3,
            },
        ];
        assert_eq!(
            ("1. workout\n2. poop\n3. code").to_string(),
            format_list(&tasks)
        );
    }

    #[test]
    fn test_delete_task() {
        let mut tasks: Vec<Task> = vec![
            Task {
                name: String::from("workout"),
                position: 1,
                quadrant: 1,
            },
            Task {
                name: String::from("poop"),
                position: 2,
                quadrant: 2,
            },
            Task {
                name: String::from("code"),
                position: 3,
                quadrant: 3,
            },
        ];

        delete_task(&mut tasks, 2);

        assert_eq!(
            tasks,
            vec![
                Task {
                    name: String::from("workout"),
                    position: 1,
                    quadrant: 1,
                },
                Task {
                    name: String::from("code"),
                    position: 2,
                    quadrant: 3,
                },
            ]
        )
    }

    #[test]
    fn test_complete_task() {
        let mut tasks: Vec<Task> = vec![
            Task {
                name: String::from("workout"),
                position: 1,
                quadrant: 1,
            },
            Task {
                name: String::from("poop"),
                position: 2,
                quadrant: 2,
            },
            Task {
                name: String::from("code"),
                position: 3,
                quadrant: 3,
            },
        ];

        complete_task(&mut tasks);
        assert_eq!(
            tasks,
            vec![
                Task {
                    name: String::from("poop"),
                    position: 1,
                    quadrant: 2,
                },
                Task {
                    name: String::from("code"),
                    position: 2,
                    quadrant: 3,
                },
            ]
        )
    }
}
