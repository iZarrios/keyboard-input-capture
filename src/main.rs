use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use eframe::egui;

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

            // src: https://github.com/torvalds/linux/blob/528dd46d0fc35c0176257a13a27d41e44fcc6cb3/include/uapi/linux/input-event-codes.h#L39
            const EV_KEY: u16 = 1;

            const VALUE_KEY_UP: i32 = 0;
            // const VALUE_KEY_DOWN: i32 = 1;
            // const VALUE_KEY_REPEAT: i32 = 2;

            if event.code == 0 || event.typ != EV_KEY || event.value == VALUE_KEY_UP {
                println!("Skipped, {:?}", event);
                continue;
            }
            println!("{:?}", event);
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

    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options
        .viewport
        .with_transparent(true)
        .with_decorations(false);
    let _ = eframe::run_native(
        "Keyboard Interface",
        native_options,
        Box::new(move |cc| Ok(Box::new(KeyPrinter::new(cc, key_code_rx, ctx_tx)))),
    );

    // let _ = t.join();
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
        cc.egui_ctx
            .style_mut(|style| style.visuals.window_fill = egui::Color32::TRANSPARENT);
        cc.egui_ctx.style_mut(|style| {
            style.visuals.panel_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 127)
        });

        return KeyPrinter {
            rx,
            pressed_key_codes: VecDeque::<u16>::new(),
        };
    }
}

impl eframe::App for KeyPrinter {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        while let Ok(event) = self.rx.try_recv() {
            if self.pressed_key_codes.len() > 20 {
                let _ = self.pressed_key_codes.pop_front();
            }
            self.pressed_key_codes.push_back(event.code);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
            for code in &self.pressed_key_codes {
                let rich_text = egui::RichText::new(code_to_char(*code))
                    .family(egui::FontFamily::Monospace)
                    .color(egui::Color32::WHITE)
                    .size(40.0);
                ui.label(rich_text);
            }
        });
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
        12 => "-",
        13 => "=",
        14 => "BACKSPACE",
        15 => "<Tab>",
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
        26 => "[",
        27 => "]",
        28 => "ENTER",
        29 => "L-CTL",
        30 => "A",
        31 => "S",
        32 => "D",
        33 => "F",
        34 => "G",
        35 => "H",
        36 => "J",
        37 => "K",
        38 => "L",
        39 => ";",
        40 => "'",
        41 => "`",
        42 => "L-Shift",
        43 => "\\",
        44 => "Z",
        45 => "X",
        46 => "C",
        47 => "V",
        48 => "B",
        49 => "N",
        50 => "M",
        51 => ",",
        52 => ".",
        53 => "/",
        54 => "R-Shift",
        55 => "KPASTERISK",
        56 => "L-Alt",
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
        103 => "UP",
        105 => "LEFT",
        106 => "RIGHT",
        108 => "DOWN",
        _ => {
            // println!("Unknown code: {}", code); // Log the unexpected value
            return "<UNK>";
        }
    }
}
