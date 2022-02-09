mod graph;
mod utils;

use gtk::prelude::*;
use gtk::{self, Application, Button};
use graph::Graph;
use graph::Connector;

fn main() {
    let grapher = Application::new(Some("org.wsn.frontend"),
    gio::ApplicationFlags::FLAGS_NONE);

    grapher.connect_activate(app);
    grapher.run();
}

fn app(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_size_request(600, 400);
    window.set_title("Graphing Test");

    let (data_graph, button) = utils::connect_graph(Graph::new(), Button::builder().label("Add data").build());
    data_graph.connect_to_window_events();

    let gui_box = gtk::Box::new(gtk::Orientation::Vertical, 15);


    gui_box.pack_start(&data_graph.borrow().layout, true, true, 15);
    gui_box.pack_end(&button, false, true, 5);
    window.add(&gui_box);
    window.show_all();
}

