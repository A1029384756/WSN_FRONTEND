mod graph;
mod utils;
mod bluetooth;
use graph::Connector;
use graph::Graph;
use gtk::prelude::*;
use gtk::{self, Application, Button};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let grapher = Application::new(Some("org.wsn.frontend"), gio::ApplicationFlags::FLAGS_NONE);

    grapher.connect_activate(app);    
    grapher.run();

    utils::disconnect_bluetooth().await;
    
    Ok(())
}

fn app(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_size_request(600, 400);
    window.set_title("Wireless Sensor Node Viewer");

    let (tx, rx) = glib::MainContext::channel::<f64>(glib::PRIORITY_DEFAULT);

    let mut blueman = bluetooth::BluetoothManager{ module: None, tx: Some(tx) };

    let bluetooth_task = tokio::spawn(async move {
        blueman.connect_module().await.unwrap();
        blueman.subscribe_to_temp().await.unwrap();
        blueman.bluetooth_handler().await;
    });

    let (data_graph, button) =
        utils::connect_graph(Graph::new(), Button::builder().label("Add data").build(), rx);
    data_graph.connect_to_window_events();

    let gui_box = gtk::Box::new(gtk::Orientation::Vertical, 15);

    gui_box.pack_start(&data_graph.borrow().layout, true, true, 15);
    gui_box.pack_end(&button, false, true, 5);
    window.add(&gui_box);

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(move |_,_| {
        bluetooth_task.abort();
    });

    window.show_all();
}