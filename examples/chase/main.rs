#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

use embedded_graphics::primitives::Rectangle;

const MUSIC: &[u8] = include_bytes!("jojo.mp3");
const PUNCH: &[u8] = include_bytes!("punch.mp3");

#[derive(Default)]
struct Player {
    x: i32,
    y: i32,
    size: u32,
}

impl Player {
    fn collides(&self, enemy: &Enemy) -> bool {
        self.x < enemy.x + enemy.size as i32
            && self.x + self.size as i32 > enemy.x
            && self.y < enemy.y + enemy.size as i32
            && self.y + self.size as i32 > enemy.y
    }
}

#[derive(Default)]
struct Enemy {
    x: i32,
    y: i32,
    size: u32,
    color: u8,
    dead: bool,
}

#[derive(Default)]
struct Game {
    player: Player,
    enemies: Vec<Enemy>,
}

impl State for Game {
    fn init() {
        Mp3Player::play_buffer(MUSIC);
    }

    fn update(&mut self) {
        if Pad::buttons_down(Controller::One) == Button::Start {
            unsafe { ogc::ffi::exit(0); };
        }

        let (stick_x, stick_y) = (Pad::stick_x(Controller::One), Pad::stick_y(Controller::One));

        self.player.x += (stick_x / (i8::MAX / 8)) as i32;
        self.player.y -= (stick_y / (i8::MAX / 8)) as i32;
        self.player.x = self.player.x.max(0);
        self.player.x = self.player.x.min(640 - self.player.size as i32);
        self.player.y = self.player.y.max(0);
        self.player.y = self.player.y.min(528 - (self.player.size * 2) as i32);

        for enemy in self.enemies.iter_mut().filter(|e| !e.dead) {
            if !self.player.collides(enemy) {
                enemy.x -= (enemy.x - (self.player.x + 25)) / enemy.size as i32;
                enemy.y -= (enemy.y - (self.player.y + 25)) / enemy.size as i32;
            } else {
                if Pad::buttons_down(Controller::One) == Button::A {
                    enemy.color = enemy.color.saturating_sub(32);
                    Mp3Player::stop();
                    Mp3Player::play_buffer(PUNCH);

                    if enemy.color == 0 {
                        enemy.dead = true;
                    }
                }
            }
        }

        if self.enemies.iter().all(|e| e.dead) {
            let mut new_enemy = Enemy::default();
            new_enemy.size = self.enemies.last().unwrap().size + 25;
            new_enemy.color = 255;
            self.enemies.push(new_enemy);

            for enemy in self.enemies.iter_mut() {
                enemy.dead = false;
                enemy.color = 255;
            }
        }

        if !Mp3Player::is_playing() {
            Mp3Player::play_buffer(MUSIC);
        }
    }

    fn draw(&self, display: &mut Display) -> Result<(), DrawError> {
        display.clear(Rgb::new(0, 100, 150))?;

        let rectangle = Rectangle::new(
            Point::new(self.player.x, self.player.y),
            Size::new(self.player.size, self.player.size),
        );
        display.fill_solid(&rectangle, Rgb::GREEN)?;

        for enemy in self.enemies.iter().filter(|e| !e.dead) {
            let rectangle = Rectangle::new(
                Point::new(enemy.x, enemy.y),
                Size::new(enemy.size, enemy.size),
            );
            let color = Rgb::new(enemy.color, enemy.color.saturating_sub(50), enemy.color);
            display.fill_solid(&rectangle, color)?;
        }

        Ok(())
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut state = Game::default();
    let mut enemy = Enemy::default();
    enemy.size = 30;
    enemy.color = 255;
    state.enemies.push(enemy);
    state.player.size = 50;
    Engine::run(state)
}
