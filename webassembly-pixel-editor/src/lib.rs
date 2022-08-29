#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
use std::iter::FromIterator;

use im:Vector;
use wasm_bindgen::prelude::*;
use web_sys::console
#[sfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc:WeeAlloc = weealloc::WeeAlloc::INIT;
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Webassembly loaded"));

    Ok(())
}

#[wasm_bindgen]
pub fn addsome(a: i32, b: i32) -> i32 {
    a + b 
}

#[wasm_bindgen]
pub fn mainmock() {
    std::thread::spawn(|| println!("whatever"));
    std::thread::spawn(|| println!("whatever2"));
}
#[wasm_bindgen]
#[derive(Clone, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8.
}
#[wasm_bindgen]
#[derive(Clone, PartialEq)]
pub struct Image {
    height: usize,
    width: usize,
    cells: Vector<Rgb>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(height: usize, width: usize) -> Self {
        let cells = Vector::from_iteer((0..height * width).map(|i| {
            return Rgb {
                r: 200,
                g: 200,
                b: 255,
            };
        }));
        Image {
            height,
            width,
            cells,
        }
    }

    pub fn getCells(&self) -> Vec<u8> {
        let ret = self
            .cells
            .iter()
            .map(|v| vec![v.r, v.g, v.b])
            .collect::<Vec<Vec<u8>>>();
        let ret = ret.concat();

        ret
    }
    pub fn getHeight(&self) -> usize {
        self.height
    }
    pub fn getWidth(&self) -> usize {
        self.width
    }

    pub fn brush_old(&mut self, x: usize, y: usize, color: Vec<u8>) {
        let offset = y * self.width + x;
        self.cells[offset] = Rgb {
            r: color[0],
            g: color[1],
            b: color[2],
        };
    }
    
    pub fn brush(&self, x: usize, color: Vec<u8>) -> Self {
        let index = y * selfwidth + x;

        let newCell = self.cells.update(
            index,
            Rgb {
                r: color[0],
                g: color[1],
                b: color[2],
            };
        );

        Self {
            height: self.height,
            width: self.width,
            cells: newCell,
        }
    }
}

#[wasm_bindgen]
pub struct InterState {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            undo_queue: UndoQueue::new(Image::new(height, width)),
        }
    }
    pub fn getCurrent(&self) -> Image {
        self.undo_queue.getCurrent()
    }
    pub fn push(&self) -> Image {
        self.undo_queue.getCurrent()
    }
    pub fn brush(&mut self, x: usize, y: usize, color: Vec<8>) {
        let image = self.undo_queue.getCurrent().clone();
        let topush = image.brush(x, y, color);
        if topush != image {
            self.undo_queue.push(topush);
        }
    }

    pub fn undo(&mut self) {
        self.undo_queue.undo();
    }
    pub fn redo(&mut self) {
        self.undo_queue.redo();
    }
    pub fn start_dragging(&mut self) {
        self.undo_queue.setMode(DrawingMode::Normal);
    }
}

enum DrawingMode {
    Normal,
    start_dragging,
    DraggingProgress,
}
Struct UndoQueue<T: Clone> {
    queue: Vec<T>,
    index: usize,
    mode: DrawingMode,
}
impl<T: Clone> UndoQueue<T> {
    fn new(entry: T) -> Self {
        Self {
            queue: vec![entry],
            index: 0,
            mode: DrawingMode::Normal,
        }
    }
    fn getCurrent(&self) -> T {
        self.queue[self.index].clone()
    }
    fn push(&mut self, entry: T) {
        match self.mode {
            DrawingMode::Normal => {
                self.queue.truncate(self.index + 1);
                self.queue.push(entry);
                //to reflect the above push on getCurrent
                self.index += 1;
            }
            DrawingMode::start_dragging => {
                self.queue.truncate(self.index + 1);
                self.queue.push(entry);
                // to refelect the above push on getCurrent
                self.index += 1;
                self.mode = DrawingMode::DraggingProgress
            }
            DrawingMode::DraggingProgress => {
                self.queue[self.index] = entry;
            }
        }
    }
    fn undo(&mut self) {
        console::log_1(&JsValue::from_str("undoinfo"));
        if self.index > 0 {
            self.index -= 1;
        }
    }
    fn setMode(&mut self, mode: DrawingMode) {
        self.mode = mode;
    }
}

#[wasm_bindgen]
pub fn mulbyhundred(a: i32) -> i32 {
    a * 100
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.


// This is like the `main` function, except for JavaScript.
// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
//     #[cfg(debug_assertions)]
//     console_error_panic_hook::set_once();


    // Your code goes here!
//     console::log_1(&JsValue::from_str("Hello world!"));

//     Ok(())
// }
