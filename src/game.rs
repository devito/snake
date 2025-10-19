// src/game.rs
use crate::{snake::Snake, snake::Point, palette::set_draw_color, wasm4};
//use crate::wasm4;

const FRUIT_SPRITE: [u8; 16] = [ 0x00,0xa0,0x02,0x00,0x0e,0xf0,0x36,0x5c,0xd6,0x57,0xd5,0x57,0x35,0x5c,0x0f,0xf0 ];

// Simple LCG random number generator
struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
    
    fn range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as u32;
        min + ((self.next() % range) as i32)
    }
}

pub struct Game {
    rng: SimpleRng,
    snake: Snake,
    frame_count: u32,
    prev_gamepad: u8,
    fruit: Point,
}

impl Game {
    pub fn new() -> Self {
      let mut rng = SimpleRng::new(235);
      Self {
          snake: Snake::new(),
          frame_count: 0,
          prev_gamepad: 0,
          fruit: Point { x: rng.range(0, 20), y: rng.range(0, 20) },
          rng,
      }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        self.input();

        if self.snake.is_dead() {
            wasm4::tone(262, 30, 60, wasm4::TONE_PULSE1);
            self.snake = Snake::new();
            self.frame_count = 0;
            self.prev_gamepad = 0;
            self.fruit.x = self.rng.range(0, 20);
            self.fruit.y = self.rng.range(0, 20);
        }

        if self.frame_count %15 == 0 {
            let dropped_pos = self.snake.update();

            if self.snake.body[0] == self.fruit {
                if let Some(last_pos) = dropped_pos {
                    wasm4::tone(730, 20, 40, wasm4::TONE_PULSE1);
                    self.snake.body.push(last_pos);
                }

                self.fruit.x = self.rng.range(0, 20);
                self.fruit.y = self.rng.range(0, 20);
            }
        }

        self.snake.draw();

        set_draw_color(0x4320);
        wasm4::blit(
            &FRUIT_SPRITE,
            self.fruit.x * 8,
            self.fruit.y *8,
            8,
            8,
      wasm4::BLIT_2BPP);
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.snake.up();
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.snake.right();
        }
        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.snake.left();
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.snake.down();
        }

        self.prev_gamepad = gamepad;
    }
}