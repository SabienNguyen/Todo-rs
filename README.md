# Todo-rs
 [WIP] A fast, easy-to-use, self-organizing to-do list built in Rust. Can currently do all basic features

 ## My desire for this todo list:
I have used a few applications to create a to-do list. At the top of the complexity spectrum is Notion, which for me ended up being too much. Then I went to the bottom of the complexity spectrum, which is just building a to-do list out of a notes app. That was simple to start but just ended up not having enough capabilities for it to be useful. Over the years, the best to-do list I used was Todoist. Todoist is built to implement any type of task organization philosophy. Taking inspiration from that design, I want to build an easy-to-use to-do list that implements an organization philosophy that I commonly use. Hopefully, this to-do list can become the main tool that I use!
 ## Basic features
 - Add tasks
 - Delete tasks
 - Complete tasks
 - Display task list
 
 ## Advanced features
 - Sort tasks based on 4 quadrants of task categorization
    - tasks to finish feature
      - rewrite struct to contain a quadrant value
      - implement sorting function   
      - develop personal hierarchy on event order
 - keep memory of task list locally
    - I think this is possible by just saving list to text file then reading and writing to it

## Final features
  - clean ui
    - use external crate perhaps
  - commands simple
    - this includes making commands more intuitive for example doing add [task name]
  - optimize/refactor code
    - try to rewrite in idomatic rust code where I can
  - use list anywhere and save in cloud or in some way communicate task file across devices

