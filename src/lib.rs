pub fn add_task(list: &mut Vec<String>, new_task: String) {
    list.push(new_task);
}

pub fn format_list(list: &Vec<String>) -> String {
    list.iter()
        .enumerate()
        .map(|(i, item)| format!("{}. {}", i + 1, item))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, String::from("workout"));
        assert_eq!(tasks, vec!["workout"]);
    }

    #[test]
    fn test_display_list() {
        let tasks: Vec<String> = vec![
            String::from("workout"),
            String::from("code"),
            String::from("poop"),
        ];
        assert_eq!(
            ("1. workout\n2. code\n3. poop").to_string(),
            format_list(&tasks)
        );
    }
}
