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

#[derive(Default)]
struct Enemy {
    x: i32,
    y: i32,
    size: u32,
    color: u8,
    dead: bool,
}

fn is_collision(rect1: &Player, rect2: &Enemy) -> bool {
    rect1.x < rect2.x + rect2.size as i32
        && rect1.x + rect1.size as i32 > rect2.x
        && rect1.y < rect2.y + rect2.size as i32
        && rect1.y + rect1.size as i32 > rect2.y
}

#[derive(Default)]
struct Game {
    player: Player,
    enemies: Vec<Enemy>,
}

impl State for Game {
    fn setup(&mut self, _video: &mut Video) {
        Pad::init();
        Mp3Player::play_buffer(MUSIC);
    }

    fn update(&mut self, _video: &mut Video, display: &mut Display) {
        Pad::scan_pads();
        display.clear(Rgb::new(0, 100, 150)).unwrap();

        let (player, enemies) = (&mut self.player, &mut self.enemies);

        // PLAYER CODE
        let rectangle = Rectangle::new(
            Point::new(player.x, player.y),
            Size::new(player.size, player.size),
        );
        display.fill_solid(&rectangle, Rgb::WHITE).unwrap();

        let (stick_x, stick_y) = (Pad::stick_x(Controller::One), Pad::stick_y(Controller::One));

        player.x += (stick_x / (i8::MAX / 8)) as i32;
        player.y -= (stick_y / (i8::MAX / 8)) as i32;

        player.x = player.x.max(0);
        player.x = player.x.min(640 - player.size as i32);
        player.y = player.y.max(0);
        player.y = player.y.min(528 - (player.size * 2) as i32);

        // ENEMY CODE
        for enemy in enemies.iter_mut().filter(|e| !e.dead) {
            let rectangle = Rectangle::new(
                Point::new(enemy.x, enemy.y),
                Size::new(enemy.size, enemy.size),
            );
            display
                .fill_solid(
                    &rectangle,
                    Rgb::new(enemy.color, enemy.color.saturating_sub(50), enemy.color),
                )
                .unwrap();

            if !is_collision(player, enemy) {
                enemy.x -= (enemy.x - (player.x + 25)) / enemy.size as i32;
                enemy.y -= (enemy.y - (player.y + 25)) / enemy.size as i32;
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

        if enemies.iter().all(|e| e.dead) {
            let mut new_enemy = Enemy::default();
            new_enemy.size = enemies.last().unwrap().size + 25;
            new_enemy.color = 255;
            enemies.push(new_enemy);

            for enemy in enemies.iter_mut() {
                enemy.dead = false;
                enemy.color = 255;
            }
        }

        // MUSIC
        if !Mp3Player::is_playing() {
            Mp3Player::play_buffer(MUSIC);
        }
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
    Engine::new().state(Box::new(state)).run()
}
