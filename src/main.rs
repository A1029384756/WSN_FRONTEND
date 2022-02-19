#![feature(once_cell)]

mod graph;
mod utils;
mod bluetooth;

use graph::Connector;
use graph::Graph;
use gtk::prelude::*;
use gtk::{self, Application, Button};
use std::error::Error;
use std::lazy::SyncLazy;

use tokio::runtime::{Builder as RuntimeBuilder, Runtime};
pub static RUNTIME: SyncLazy<Runtime> = SyncLazy::new(|| {
    RuntimeBuilder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap()
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let grapher = Application::new(Some("org.wsn.frontend"), gio::ApplicationFlags::FLAGS_NONE);

    grapher.connect_activate(app);
    
    grapher.run();
    
    Ok(())
}

fn app(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_size_request(600, 400);
    window.set_title("Wireless Sensor Node Viewer");

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak app => move |_,_| {
        
        app.quit();
    }));

    let (tx, rx) = glib::MainContext::channel::<f64>(glib::PRIORITY_DEFAULT);

    tokio::spawn(async move {
        let module = bluetooth::connect_module().await.expect("Could not connect to module");
        bluetooth::subscribe_to_temp(&module).await.unwrap();
        bluetooth::bluetooth_handler(module, tx).await;
    });


    let (data_graph, button) =
        utils::connect_graph(Graph::new(), Button::builder().label("Add data").build(), rx);
    data_graph.connect_to_window_events();

    let gui_box = gtk::Box::new(gtk::Orientation::Vertical, 15);

    gui_box.pack_start(&data_graph.borrow().layout, true, true, 15);
    gui_box.pack_end(&button, false, true, 5);
    window.add(&gui_box);

    window.show_all();
}