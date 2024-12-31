pub fn add_task(list: &mut Vec<String>, new_task: String) {
    list.push(new_task);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    #[test]
    fn test_add_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, String::from("workout"));
        assert_eq!(tasks, vec!["workout"]);
    }

    // #[test]
    // fn test_display_list() {
    //     let tasks = vec!["workout", "code", "poop"];
    //     let mut output = Vec::new();
    //     // Redirecting stdout to the buffer
    //     let _ = io::set_boxed_writer(&mut output);

    //     // Call the function
    //     display_list(&tasks);

    //     // Restore stdout
    //     let _ = io::set_boxed_writer(Box::new(io::stdout()));

    //     let output = String::from_utf8_lossy(&output);

    //     // Check the output
    //     assert!(output.contains(("workout\ncode\npoop").lines().map(|line| line.to_string()).collect()));
    // }
}
