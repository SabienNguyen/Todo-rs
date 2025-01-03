#[derive(Debug)]
pub struct Task {
    name: String,
    position: u8,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.position == other.position
    }
}

pub fn add_task(list: &mut Vec<Task>, new_task: String) {
    list.push(Task {
        name: new_task,
        position: (list.len() + 1) as u8,
    });
}

fn organize_tasks(list: &mut Vec<Task>) {
    let _ = list
        .iter_mut()
        .enumerate()
        .map(|(i, task)| task.position = (i + 1) as u8)
        .collect::<Vec<_>>();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, String::from("workout"));
        assert_eq!(
            tasks,
            vec![Task {
                name: String::from("workout"),
                position: 1
            }]
        );
    }

    #[test]
    fn test_display_list() {
        let tasks: Vec<Task> = vec![
            Task {
                name: String::from("workout"),
                position: 1,
            },
            Task {
                name: String::from("code"),
                position: 2,
            },
            Task {
                name: String::from("poop"),
                position: 3,
            },
        ];
        assert_eq!(
            ("1. workout\n2. code\n3. poop").to_string(),
            format_list(&tasks)
        );
    }

    #[test]
    fn test_delete_task() {
        let mut tasks: Vec<Task> = vec![
            Task {
                name: String::from("workout"),
                position: 1,
            },
            Task {
                name: String::from("code"),
                position: 2,
            },
            Task {
                name: String::from("poop"),
                position: 3,
            },
        ];

        delete_task(&mut tasks, 2);

        assert_eq!(
            tasks,
            vec![
                Task {
                    name: String::from("workout"),
                    position: 1,
                },
                Task {
                    name: String::from("poop"),
                    position: 2,
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
            },
            Task {
                name: String::from("code"),
                position: 2,
            },
            Task {
                name: String::from("poop"),
                position: 3,
            },
        ];

        complete_task(&mut tasks);
        assert_eq!(
            tasks,
            vec![
                Task {
                    name: String::from("code"),
                    position: 1,
                },
                Task {
                    name: String::from("poop"),
                    position: 2,
                },
            ]
        )
    }
}
