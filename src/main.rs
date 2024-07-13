use eframe::egui;
use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

#[repr(C)]
#[derive(Debug)]
struct InputEvent {
    pad: [u64; 2],
    typ: u16,
    code: u16,
    value: i32,
}

impl InputEvent {
    pub fn new() -> InputEvent {
        return InputEvent {
            pad: [0; 2],
            typ: 0,
            code: 0,
            value: 0,
        };
    }
}

fn reader_thread(tx: Sender<InputEvent>, rx: Receiver<egui::Context>) -> () {
    let ctx = rx.recv().unwrap();

    let mut f = File::open("/dev/input/by-id/usb-Evision_RGB_Keyboard-event-kbd").unwrap();

    unsafe {
        loop {
            let mut event = InputEvent::new();
            {
                let event_buf = std::slice::from_raw_parts_mut(
                    &mut event as *mut _ as *mut u8,
                    core::mem::size_of::<InputEvent>(),
                );
                f.read_exact(event_buf).unwrap();
            }
            // println!("{:?}", event);
            tx.send(event).unwrap();
            ctx.request_repaint();
        }
    }
}

fn main() {
    let (key_code_tx, key_code_rx) = mpsc::channel();
    let (ctx_tx, ctx_rx) = mpsc::channel();
    // we use move here to consume it

    let t = thread::spawn(move || reader_thread(key_code_tx, ctx_rx));

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Keyboard Interface",
        native_options,
        Box::new(move |cc| Ok(Box::new(KeyPrinter::new(cc, key_code_rx, ctx_tx)))),
    );
}

struct KeyPrinter {
    rx: Receiver<InputEvent>,
    pressed_key_codes: VecDeque<u16>,
}

impl KeyPrinter {
    fn new(
        cc: &eframe::CreationContext<'_>,
        rx: Receiver<InputEvent>,
        tx: Sender<egui::Context>,
    ) -> Self {
        tx.send(cc.egui_ctx.clone()).unwrap();
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        return KeyPrinter {
            rx,
            pressed_key_codes: VecDeque::<u16>::new(),
        };
    }
}

impl eframe::App for KeyPrinter {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        while let Ok(event) = self.rx.try_recv() {
            self.pressed_key_codes.push_back(event.code);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            for code in &self.pressed_key_codes {
                ui.label(code_to_char(*code));
            }
            ui.heading("Hello World!");
        });
    }
}
fn code_to_char(code: u16) -> &'static str {
    match code {
        1 => "ESC",
        2 => "1",
        3 => "2",
        4 => "3",
        5 => "4",
        6 => "5",
        7 => "6",
        8 => "7",
        9 => "8",
        10 => "9",
        11 => " 0",
        12 => "MINUS",
        13 => "EQUAL",
        14 => "BACKSPACE",
        15 => "TAB",
        16 => "Q",
        17 => "W",
        18 => "E",
        19 => "R",
        20 => "T",
        21 => "Y",
        22 => "U",
        23 => "I",
        24 => "O",
        25 => "P",
        26 => "LEFTBRACE",
        27 => "RIGHTBRACE",
        28 => "ENTER",
        29 => "LEFTCTRL",
        30 => "A",
        31 => "S",
        32 => "D",
        33 => "F",
        34 => "G",
        35 => "H",
        36 => "J",
        37 => "K",
        38 => "L",
        39 => "SEMICOLON",
        40 => "APOSTROPHE",
        41 => "GRAVE",
        42 => "LEFTSHIFT",
        43 => "BACKSLASH",
        44 => "Z",
        45 => "X",
        46 => "C",
        47 => "V",
        48 => "B",
        49 => "N",
        50 => "M",
        51 => "COMMA",
        52 => "DOT",
        53 => "SLASH",
        54 => "RIGHTSHIFT",
        55 => "KPASTERISK",
        56 => "LEFTALT",
        57 => "SPACE",
        58 => "CAPSLOCK",
        59 => "F1",
        60 => "F2",
        61 => "F3",
        62 => "F4",
        63 => "F5",
        64 => "F6",
        65 => "F7",
        66 => "F8",
        67 => "F9",
        68 => "F10",
        69 => "NUMLOCK",
        70 => "SCROLLLOCK",
        _ => "<UNK>",
    }
}
