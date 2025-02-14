use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use todo_list::{add_task, complete_task, delete_task, load_task_list, save_task_list, Task};

async fn get_tasks() -> impl Responder {
    let tasks = load_task_list().unwrap_or_default();
    HttpResponse::Ok().json(tasks)
}

async fn add_new_task(new_task: web::Json<Task>) -> impl Responder {
    let mut tasks = load_task_list().unwrap_or_default();
    add_task(&mut tasks, new_task.get_name(), new_task.get_quadrant());
    save_task_list(&tasks).unwrap();
    HttpResponse::Ok().json(tasks)
}

async fn remove_task(info: web::Path<u8>) -> impl Responder {
    let mut tasks = load_task_list().unwrap_or_default();
    delete_task(&mut tasks, *info);
    save_task_list(&tasks).unwrap();
    HttpResponse::Ok().json(tasks)
}

async fn mark_task_complete() -> impl Responder {
    let mut tasks = load_task_list().unwrap_or_default();
    complete_task(&mut tasks);
    save_task_list(&tasks).unwrap();
    HttpResponse::Ok().json(tasks)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .max_age(3600)
                )
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks/add", web::get().to(add_new_task))
            .route("/tasks/delete/{position}", web::get().to(remove_task))
            .route("/tasks/complete", web::post().to(mark_task_complete))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
