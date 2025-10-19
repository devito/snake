#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod wasm4;
mod palette;
mod snake;
use game::Game;
use core::cell::UnsafeCell;

// #[rustfmt::skip]
// const SMILEY: [u8; 8] = [
//     0b11000011,
//     0b10000001,
//     0b00100100,
//     0b00100100,
//     0b00000000,
//     0b00100100,
//     0b10011001,
//     0b11000011,
// ];

static mut SNAKE_GAME: UnsafeCell<Option<Game>> = UnsafeCell::new(None);

#[no_mangle]
fn start() {
    palette::set_palette([0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d]);
    unsafe {
        *SNAKE_GAME.get() = Some(Game::new());
    }
}

#[no_mangle]
fn update() {
    unsafe {
        if let Some(game) = (*SNAKE_GAME.get()).as_mut() {
            game.update();
        }
    }
//trace("Hello world!");
    // unsafe { *DRAW_COLORS = 2 }
    // text("Hello from Rust!", 10, 10);

    // let gamepad = unsafe { *GAMEPAD1 };
    // if gamepad & BUTTON_1 != 0 {
    //     unsafe { *DRAW_COLORS = 4 }
    // }

    // blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    // text("Press X to blink", 16, 90);
}
