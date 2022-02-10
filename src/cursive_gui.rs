/////// ALL CODE SO FAR EXAMPLE CODE FROM 
/// https://github.com/gyscos/cursive/tree/main/cursive/examples
/////////////////////////////////////////
use cursive::traits::*;
use cursive::Vec2;
use cursive::{Cursive, Printer};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use cursive::theme::{Color, ColorStyle};
use cursive::view::Resizable;
use cursive::views::Canvas;
use cursive::views::{Button, DummyView, LinearLayout, Panel};

// This example will print a stream of logs generated from a separate thread.
//
// We will use a custom view using a channel to receive data asynchronously.

pub fn gui_main() {
    // As usual, create the Cursive root
    let mut siv = cursive::default();

    siv.load_toml(include_str!("assets/style.toml")).unwrap();
    

    let cb_sink = siv.cb_sink().clone();

    // We want to refresh the page even when no input is given.
    siv.add_global_callback('q', |s| s.quit());

    // A channel will communicate data from our running task to the UI.
    let (tx, rx) = mpsc::channel();

    // Generate data in a separate thread.
    thread::spawn(move || {
        generate_logs(&tx, cb_sink);
    });

    // And sets the view to read from the other end of the channel.
    siv.add_layer(
        LinearLayout::vertical()
            .child(
                BufferView::new(200, rx)
                .fixed_height(10)
                .full_width()
                .scrollable(),
            )
            .child(
                Canvas::new(())
                .with_draw(draw)
                .fixed_size((20, 10))
                ,
            )
    );

    

    siv.run();
}

// We will only simulate log generation here.
// In real life, this may come from a running task, a separate process, ...
fn generate_logs(tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink) {
    let mut i = 1;
    loop {
        let line = format!("Interesting log line {}", i);
        i += 1;
        // The send will fail when the other side is dropped.
        // (When the application ends).
        if tx.send(line).is_err() {
            return;
        }
        cb_sink.send(Box::new(Cursive::noop)).unwrap();
        thread::sleep(Duration::from_millis(30));
    }
}

// Let's define a buffer view, that shows the last lines from a stream.
struct BufferView {
    // We'll use a ring buffer
    buffer: VecDeque<String>,
    // Receiving end of the stream
    rx: mpsc::Receiver<String>,
}

impl BufferView {
    // Creates a new view with the given buffer size
    fn new(size: usize, rx: mpsc::Receiver<String>) -> Self {
        let mut buffer = VecDeque::new();
        buffer.resize(size, String::new());
        BufferView { buffer, rx }
    }

    // Reads available data from the stream into the buffer
    fn update(&mut self) {
        // Add each available line to the end of the buffer.
        while let Ok(line) = self.rx.try_recv() {
            self.buffer.push_back(line);
            self.buffer.pop_front();
        }
    }
}

impl View for BufferView {
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the end of the buffer
        for (i, line) in
            self.buffer.iter().rev().take(printer.size.y).enumerate()
        {
            printer.print((0, printer.size.y - 1 - i), line);
        }
    }
}


/// Method used to draw the cube.
///
/// This takes as input the Canvas state and a printer.
fn draw(_: &(), p: &Printer) {
    // We use the view size to calibrate the color
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;

    // Print each cell individually
    for x in 0..x_max {
        for y in 0..y_max {
            // We'll use a different style for each cell
            let style = ColorStyle::new(
                front_color(x, y, x_max, y_max),
                back_color(x, y, x_max, y_max),
            );

            p.with_color(style, |printer| {
                printer.print((x, y), "+");
            });
        }
    }
}

// Gradient for the front color
fn front_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
    // We return a full 24-bits RGB color, but some backends
    // will project it to a 256-colors palette.
    Color::Rgb(
        x * (255 / x_max),
        y * (255 / y_max),
        (x + 2 * y) * (255 / (x_max + 2 * y_max)),
    )
}

// Gradient for the background color
fn back_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
    // Let's try to have a gradient in a different direction than the front color.
    Color::Rgb(
        128 + (2 * y_max + x - 2 * y) * (128 / (x_max + 2 * y_max)),
        255 - y * (255 / y_max),
        255 - x * (255 / x_max),
    )
}