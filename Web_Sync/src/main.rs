use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use tokio::sync::{Notify, oneshot};
use tokio::time::{sleep, Duration};

struct AppState {
    service1_called: Mutex<bool>,
    service2_called: Mutex<bool>,
    notify: Notify,
    shutdown_sender: Mutex<Option<oneshot::Sender<()>>>,
}

async fn service1(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut service1_called = data.service1_called.lock().unwrap();
    *service1_called = true;
    data.notify.notify_one();

    println!("Service 1 called. Waiting for Service 2...");
    if !*data.service2_called.lock().unwrap() {
        let timeout = sleep(Duration::from_secs(10));
        tokio::select! {
            _ = data.notify.notified() => println!("Service 2 was called. Proceeding."),
            _ = timeout => println!("Timeout reached. Proceeding without Service 2."),
        }
    }

    if *data.service2_called.lock().unwrap() {
        if let Some(sender) = data.shutdown_sender.lock().unwrap().take() {
            let _ = sender.send(());
        }
    }

    HttpResponse::Ok().body("Service 1 completed")
}

async fn service2(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut service2_called = data.service2_called.lock().unwrap();
    *service2_called = true;
    data.notify.notify_one();

    println!("Service 2 called. Waiting for Service 1...");
    if !*data.service1_called.lock().unwrap() {
        let timeout = sleep(Duration::from_secs(10));
        tokio::select! {
            _ = data.notify.notified() => println!("Service 1 was called. Proceeding."),
            _ = timeout => println!("Timeout reached. Proceeding without Service 1."),
        }
    }

    if *data.service1_called.lock().unwrap() {
        if let Some(sender) = data.shutdown_sender.lock().unwrap().take() {
            let _ = sender.send(());
        }
    }

    HttpResponse::Ok().body("Service 2 completed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();

    let app_state = Arc::new(AppState {
        service1_called: Mutex::new(false),
        service2_called: Mutex::new(false),
        notify: Notify::new(),
        shutdown_sender: Mutex::new(Some(shutdown_sender)),
    });

    println!("Server starting on http://127.0.0.1:8080");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/service1", web::post().to(service1))
            .route("/service2", web::post().to(service2))
    })
    .bind("127.0.0.1:8080")?
    .run();

    tokio::select! {
        _ = server => println!("Server stopped"),
        _ = shutdown_receiver => {
            println!("Both services called. Shutting down.");
            std::process::exit(0);
        }
    }

    Ok(())
}