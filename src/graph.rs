use gtk::{prelude::*, Viewport};
use gtk::{self, cairo, DrawingArea};
use std::cell::RefCell;

use std::rc::Rc;
use crate::utils::first_vec_element;

pub struct Graph {
    pub data: Vec<f64>,
    pub layout: gtk::Box,
    pub canvas: DrawingArea,
    pub viewport: Viewport,
    initial_diff: Option<i32>,
    deleted_element: f64,
}

impl Graph {
    pub fn new() -> Graph {
        let mut g = Graph {
            data: Vec::with_capacity(30),
            layout: gtk::Box::builder().margin(15).build(),
            canvas: DrawingArea::builder().margin(15).build(),
            viewport: Viewport::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>),
            initial_diff: None,
            deleted_element: 0.0,
        };
        println!("Created new graph");
        g.fill_with_data();
        g.canvas.set_size_request(500,300);
        g.viewport.add(&g.canvas);
        g.layout.pack_start(&g.viewport,true,true,0);
        g
    }

    fn fill_with_data(&mut self) {
        for i in 0..30 {
            self.data.push(rand::random::<f64>()*100.0);
            println!("Added data: {}", self.data[i]);
        }

        println!("Filled graph with data");
    }

    pub fn add_data(&mut self) {
        self.deleted_element = *first_vec_element(&self.data).unwrap();
        self.data.remove(0);
        self.data.push(rand::random::<f64>()*100.0);
        //println!("Added random data value:{:?}", self.data.last());
        self.canvas.queue_draw();
    }

    pub fn draw_graph(&self, context: &cairo::Context, width: f64, height: f64) {
        context.set_source_rgb(1.0,1.0,1.0);
        context.paint().unwrap();
        context.set_line_width(2.0);
        context.set_source_rgb(0.0,0.0,0.0);
        context.move_to(0.0, height - (height/120.0)*self.deleted_element);

        for (i, point) in self.data.iter().enumerate() {
            context.line_to((width/30.0)*((i as f64) + 1.0), height - (height/120.0)*point);
            //println!("Plotted point at Y = {}", point);
        }
        context.stroke().unwrap();

        //println!("Drew graph of size {}, {}", width, height);
    }

    pub fn send_size_request(&self, width: Option<i32>) {
        let mut width = match width {
            Some(w) => w,
            None => {
                if let Some(parent) = self.canvas.parent() {
                    parent.allocation().width() - parent.margin_start() - parent.margin_end()
                } else {
                    eprintln!(
                        "<Graph::send_size_request> A parent is required if no width is \
                               provided..."
                    );
                    return;
                }
            }
        };

        if let Some(top) = self.canvas.toplevel() {
            let max_width = top.allocation().width();
            if width > max_width {
                width = max_width;
            }
        }

        self.canvas.set_size_request(width, 300);
    }
}

pub trait Connecter {
    fn connect_to_window_events(&self);
}

impl Connecter for Rc<RefCell<Graph>> {
    fn connect_to_window_events(&self) {
        let s = self.clone();
        if let Some(parent) = self.borrow().layout.toplevel() {
            parent.connect_configure_event(move |w, _| {
                let need_diff = s.borrow().initial_diff.is_none();
                if need_diff {
                    let mut s = s.borrow_mut();
                    let parent_width = if let Some(p) = s.canvas.parent() {
                        p.allocation().width()
                    } else {
                        0
                    };
                    s.initial_diff = Some(w.allocation().width() - parent_width);
                }
                s.borrow().send_size_request(None);
                false
            });
        } else {
            eprintln!("This method needs to be called *after* it has been put inside a window");
        }
    }
}
