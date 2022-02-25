use gtk::{self, cairo, DrawingArea};
use gtk::{prelude::*, Viewport};
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::{first_vec_element, min_vec_element};
use crate::utils::max_vec_element;
use crate::utils::context_set_rgb;
use chrono::prelude::*;
use chrono::DateTime;

const GRAPH_TOP_PADDING: f64 = 25.0;
const GRAPH_BOTTOM_PADDING: f64 = 25.0;
const GRAPH_LEFT_PADDING: f64 = 25.0;
const GRAPH_RIGHT_PADDING: f64 = 75.0;

pub struct Point {
    time: DateTime<Local>,
    pub temperature: f64
}

impl Point {
    fn new(temperature: f64) -> Point {
        let point = Point{ time: Local::now(), temperature: temperature};
        point
    }
}

pub struct Graph {
    pub data: Vec<Point>,
    pub layout: gtk::Box,
    pub canvas: DrawingArea,
    pub viewport: Viewport,
    initial_diff: Option<i32>,
}

impl Graph {
    pub fn new() -> Graph {
        let mut g = Graph {
            data: Vec::with_capacity(31),
            layout: gtk::Box::builder().margin(0).build(),
            canvas: DrawingArea::builder().margin(0).build(),
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
           self.data.push(Point::new(0.0));
        }
    }

    pub fn add_data_random(&mut self) {
        self.data.remove(0);
        self.data.push(Point::new(rand::random::<f64>() * 200.0 - 100.0));
        self.canvas.queue_draw();
    }

    pub fn add_data(&mut self, temperature: f64) {
        self.data.remove(0);
        self.data.push(Point::new(temperature));
        self.canvas.queue_draw();
    }

    pub fn draw_graph(&self, context: &cairo::Context, width: f64, height: f64) {
        //Prepare time axis label
        let begin_time = format!("{}:{:0>2}:{:0>2}",
            self.data[0].time.hour().to_string(), 
            self.data[0].time.minute().to_string(), 
            self.data[0].time.second().to_string());
            
        let end_time = format!("{}:{:0>2}:{:0>2}", 
            self.data[30].time.hour().to_string(), 
            self.data[30].time.minute().to_string(), 
            self.data[30].time.second().to_string());  
        
        context_set_rgb((255, 255, 255), &context);
        context.move_to(GRAPH_LEFT_PADDING, height - 15.0);
        context.show_text(&begin_time).unwrap();
        context.move_to(width - GRAPH_RIGHT_PADDING - GRAPH_LEFT_PADDING - 20.0,
             height - GRAPH_BOTTOM_PADDING + 10.0);
        context.show_text(&end_time).unwrap();

        //Get graph parameters
        let max_val = (max_vec_element(&self.data) + 2.0 * GRAPH_TOP_PADDING)
            .max(GRAPH_TOP_PADDING+GRAPH_BOTTOM_PADDING);
        let min_val = (min_vec_element(&self.data) - 2.0 * GRAPH_BOTTOM_PADDING)
            .min(-1.0 * (GRAPH_BOTTOM_PADDING + GRAPH_TOP_PADDING));

        //Prepare Temperature Axis label
        context.move_to(width - GRAPH_RIGHT_PADDING + 5.0, height - GRAPH_BOTTOM_PADDING - 10.0);
        context.show_text(&format!("{:.2}",(min_val + GRAPH_BOTTOM_PADDING))).unwrap();
        context.move_to(width - GRAPH_RIGHT_PADDING + 5.0, GRAPH_TOP_PADDING + 10.0);
        context.show_text(&format!("{:.2}", (max_val - GRAPH_TOP_PADDING))).unwrap();

        //Set graph line color
        context.set_line_width(2.0);
        context_set_rgb((155, 89, 182), &context);

        //Prepare actual graphing
        context.move_to(
            GRAPH_LEFT_PADDING,
            height - (height / (max_val + min_val.abs())) * 
                first_vec_element(&self.data).unwrap().temperature + min_val - GRAPH_BOTTOM_PADDING
            );

        for (i, point) in self.data.iter().enumerate() {
            context.line_to(
                ((width - (GRAPH_LEFT_PADDING + GRAPH_RIGHT_PADDING)) / 30.0) * (i as f64) 
                    + GRAPH_LEFT_PADDING,

                height - (height / (max_val + min_val.abs())) * point.temperature - GRAPH_BOTTOM_PADDING + min_val,
            );
        }

        context.stroke().unwrap();

        context_set_rgb((255, 255, 255), &context);
        context.move_to(width - GRAPH_RIGHT_PADDING + 5.0, height - (height/(max_val + min_val.abs()))*self.data.last().unwrap().temperature + min_val - GRAPH_BOTTOM_PADDING);
        context.show_text(&format!("{:.2}", self.data.last().unwrap().temperature)).unwrap();

        context_set_rgb((149, 165, 166), &context);
        context.rectangle(GRAPH_LEFT_PADDING,
             GRAPH_TOP_PADDING,
              width  - GRAPH_RIGHT_PADDING - GRAPH_LEFT_PADDING,
             height - GRAPH_BOTTOM_PADDING - GRAPH_TOP_PADDING - 10.0);
        context.stroke().unwrap();

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
