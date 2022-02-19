use crate::graph::Graph;

use gtk::glib;
use gtk::prelude::*;
use gtk::Button;

use std::cell::RefCell;
use std::rc::Rc;

pub fn connect_graph(graph: Graph, button: Button, rx: glib::Receiver<f64>) -> (Rc<RefCell<Graph>>, Button) {
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
            graph.borrow_mut().add_data_random();
        }),
    );

    rx.attach(None,
        glib::clone!(@weak graph => @default-return Continue(true), move |data| {
            graph.borrow_mut().add_data(to_farenheit(data));
            Continue(true)
        }),
    );

    (graph, button)
}

pub fn first_vec_element<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

pub fn data_parse(data: &Vec<u8>) -> f64 {
    let mut value:f64;

    value = 10.0 * (data[0] - 48) as f64;
    value += (data[1] - 48) as f64;
    value += 0.1 * (data[3] - 48) as f64;
    value += 0.01 * (data[4] - 48) as f64;

    value
}

pub fn to_farenheit(mut temperature: f64) -> f64 {
    temperature = temperature * 1.8 + 32.0;
    temperature
}