use crate::graph::Graph;

use gtk::glib;
use gtk::prelude::*;
use gtk::Button;

use std::cell::RefCell;
use std::rc::Rc;

pub fn connect_graph(graph: Graph, button: Button) -> (Rc<RefCell<Graph>>, Button) {
    let canvas = graph.canvas.clone();
    let graph = Rc::new(RefCell::new(graph));
    canvas.connect_draw(
        glib::clone!(@weak graph => @default-return Inhibit(false), move |w, c| {
            graph.borrow().draw_graph(c,
                f64::from(w.allocated_width()),
                f64::from(w.allocated_height()));
            Inhibit(false)
        }),
    );
    button.connect_clicked(
        glib::clone!(@weak graph => move |_| {
            graph.borrow_mut().add_data();
        }),
    );
    (graph, button)
}

pub fn first_vec_element<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}
