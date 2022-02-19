use gtk::{self, cairo, DrawingArea};
use gtk::{prelude::*, Viewport};
use std::cell::RefCell;

use crate::utils::first_vec_element;
use std::rc::Rc;

pub struct Graph {
    pub data: Vec<f64>,
    pub layout: gtk::Box,
    pub canvas: DrawingArea,
    pub viewport: Viewport,
    initial_diff: Option<i32>,
}

impl Graph {
    pub fn new() -> Graph {
        let mut g = Graph {
            data: Vec::with_capacity(31),
            layout: gtk::Box::builder().margin(15).build(),
            canvas: DrawingArea::builder().margin(15).build(),
            viewport: Viewport::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>),
            initial_diff: None,
        };
        println!("Created new graph");
        g.fill_with_data();
        g.canvas.set_size_request(500, 300);
        g.viewport.add(&g.canvas);
        g.layout.pack_start(&g.viewport, true, true, 0);
        g
    }

    fn fill_with_data(&mut self) {
        for _i in 0..31 {
           // self.data.push(rand::random::<f64>() * 100.0);
           // println!("Added data: {}", self.data[i]);
           self.data.push(0.0);
        }

       // println!("Filled graph with data");
    }

    pub fn add_data_random(&mut self) {
        //self.deleted_element = *first_vec_element(&self.data).unwrap();
        self.data.remove(0);
        self.data.push(rand::random::<f64>() * 100.0);
        //println!("Added random data value:{:?}", self.data.last());
        self.canvas.queue_draw();
    }

    pub fn add_data(&mut self, temperature: f64) {
        //self.deleted_element = *first_vec_element(&self.data).unwrap();
        self.data.remove(0);
        self.data.push(temperature);
        //println!("Added random data value:{:?}", self.data.last());
        self.canvas.queue_draw();
    }

    pub fn draw_graph(&self, context: &cairo::Context, width: f64, height: f64) {
        context.set_source_rgb(1.0, 1.0, 1.0);
        context.paint().unwrap();
        context.set_line_width(2.0);
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.move_to(
            0.0,
            height - (height / 120.0) * first_vec_element(&self.data).unwrap(),
        );

        for (i, point) in self.data.iter().enumerate() {
            context.line_to(
                (width / 30.0) * (i as f64),
                height - (height / 120.0) * point,
            );
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
                    eprintln!("Parent required if no width provided");
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

pub trait Connector {
    fn connect_to_window_events(&self);
}

impl Connector for Rc<RefCell<Graph>> {
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
            eprintln!("Call this method after putting into a window");
        }
    }
}
