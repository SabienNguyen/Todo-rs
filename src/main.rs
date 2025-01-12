use dioxus::prelude::*;
use todo_list;
static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "container",
            Sidebar {},
            MainContent {},
        }
    }
}

#[component] 
fn Sidebar() -> Element {
    rsx! {
        div {
            class: "sidebar",
            h2 {
                "Your Profile"
            },
            ul {
                li { "Add task" }, 
                li { "Search" }, 
                li { "Inbox" }, 
                li { "Today" }, 
                li { "Upcoming" }, 
                li { "Filters & Labels" },        
            },     
            h3 {"Favorites"},
            ul {  
                li { "ASAP" },
            },
            h3 { "My Projects" },
            ul { 
                li { "Home" },
                li { "Workflow" },
                li { "Learning Machine Learning" },
                li { "bookshelf" },
             }
        }
    }
}

#[component] 
fn MainContent() -> Element {
    rsx! {
        div {
            class: "main-content",
            h1 { "Today" },
            button { "Add a new task" },
            p { "What do you need to get done today?" },
            a {
                href: "#",
                "How to plan your day."
            }
        }
    }
}