use macroquad::prelude::*;

const WIDTH: i16 = 640;
const HEIGHT: i16 = 320;

type Point = (i16, i16);

fn window_conf() -> Conf {
    Conf {
        window_title: "dasnake".to_owned(),
        fullscreen: false,
        sample_count: 0,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum State {
    Menu,
    Playing,
    Lose,
    Win,
}

struct Snake {
    pos: Point,
    last_pos: Vec<Point>,
    len: usize,
    color: Color,
    next: Point,
    last_dir: Direction,
    super_speed: bool,
}
impl Snake {
    pub fn new(pos: Point, color: Color) -> Self {
        Self {
            pos,
            last_pos: Vec::new(),
            len: 3,
            color,
            next: (1, 0),
            last_dir: Direction::Right,
            super_speed: false,
        }
    }
}

struct Game {
    canvas: Point,
    canvas_size: Point,
    tile_size: i16,
    apples: Vec<Point>,
    speed: f64,
    state: State,
}

impl Game {
    pub fn new(canvas: Point, tile_size: i16, speed: f64) -> Self {
        let canvas_size = (canvas.0 / tile_size, canvas.1 / tile_size);
        Self {
            canvas,
            canvas_size,
            tile_size,
            apples: Vec::new(),
            speed,
            state: State::Menu,
        }
    }
    pub fn spawn_apple(&mut self, snake: &Snake) {
        let pos = (
            rand::gen_range(0, self.canvas_size.0 as i32) as i16,
            rand::gen_range(0, self.canvas_size.1 as i32) as i16,
        );

        // if entire screen is full -- small possibility this is wrong lol
        if snake.last_pos.len() > (self.canvas_size.0 * self.canvas_size.1 - 1) as usize {
            self.state = State::Win;
            return;
        }

        // recursive check, works
        if snake.pos == pos || self.apples.contains(&pos) || snake.last_pos.contains(&pos) {
            self.spawn_apple(snake);
            return;
        }

        self.apples.insert(0, pos);
    }

    pub fn quit(&self) {
        std::process::exit(0);
    }

    pub fn restart(&mut self, snake: &mut Snake) {
        snake.len = 3;
        self.apples.clear();
        self.spawn_apple(snake);
        snake.pos = (0, 0);
        snake.last_pos.clear();
        snake.next = (1, 0);
        snake.last_dir = Direction::Right;
        self.state = State::Playing;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new((WIDTH, HEIGHT), 20, 0.08);
    let mut snake = Snake::new((0, 0), GREEN);
    let mut last_update = get_time();

    let up = vec![KeyCode::Up, KeyCode::W, KeyCode::K];
    let down = vec![KeyCode::Down, KeyCode::S, KeyCode::J];
    let left = vec![KeyCode::Left, KeyCode::A, KeyCode::H];
    let right = vec![KeyCode::Right, KeyCode::D, KeyCode::L];

    game.spawn_apple(&snake);

    loop {
        clear_background(BLACK);

        let elapsed = get_time() - last_update;

        match game.state {
            State::Menu => {
                text("Press Space To Begin...", 50, game.canvas);
                if is_key_pressed(KeyCode::Space) {
                    game.state = State::Playing;
                }
            }
            State::Playing => {
                {
                    // if(snake.last_key.is_some()
                    draw_rectangle(
                        (game.tile_size * snake.pos.0) as f32,
                        (game.tile_size * snake.pos.1) as f32,
                        game.tile_size as f32,
                        game.tile_size as f32,
                        snake.color,
                    );

                    for pos in snake.last_pos.iter() {
                        draw_rectangle(
                            (game.tile_size * pos.0) as f32,
                            (game.tile_size * pos.1) as f32,
                            game.tile_size as f32,
                            game.tile_size as f32,
                            snake.color,
                        );
                    }

                    for pos in game.apples.iter() {
                        draw_rectangle(
                            (game.tile_size * pos.0) as f32,
                            (game.tile_size * pos.1) as f32,
                            game.tile_size as f32,
                            game.tile_size as f32,
                            RED,
                        );
                    }
                }

                if elapsed >= game.speed || (snake.super_speed && elapsed >= game.speed / 2.) {
                    snake.pos.0 += snake.next.0;
                    snake.pos.1 += snake.next.1;

                    if snake.last_pos.contains(&snake.pos) {
                        game.state = State::Lose;
                        continue;
                    }

                    snake.pos = (
                        (snake.pos.0 + game.canvas_size.0) % game.canvas_size.0,
                        (snake.pos.1 + game.canvas_size.1) % game.canvas_size.1,
                    );

                    last_update = get_time();

                    snake.last_pos.insert(0, snake.pos);
                    snake.last_pos.truncate(snake.len);

                    game.apples.retain(|apple_pos| {
                        if apple_pos.eq(&snake.pos) {
                            snake.len += 1;
                            false
                        } else {
                            true
                        }
                    });

                    if game.apples.is_empty() {
                        game.spawn_apple(&snake);
                    }
                }

                if is_keys_down(&up) && snake.last_dir != Direction::Down {
                    snake.next = (0, -1);
                    snake.last_dir = Direction::Up;
                } else if is_keys_down(&down) && snake.last_dir != Direction::Up {
                    snake.next = (0, 1);
                    snake.last_dir = Direction::Down;
                } else if is_keys_down(&left) && snake.last_dir != Direction::Right {
                    snake.next = (-1, 0);
                    snake.last_dir = Direction::Left;
                } else if is_keys_down(&right) && snake.last_dir != Direction::Left {
                    snake.next = (1, 0);
                    snake.last_dir = Direction::Right;
                }

                snake.super_speed = is_key_down(KeyCode::Space);
            }
            State::Lose => {
                text("Sorry, you failed. RESETTING.", 40, game.canvas);
                if elapsed >= 1.0 || is_key_pressed(KeyCode::Space) {
                    game.restart(&mut snake);
                }
            }
            State::Win => {
                text("YOU DID IT? WHY? GET HELP", 40, game.canvas);
                if elapsed >= 1.0 {
                    game.quit();
                }
            }
        }

        next_frame().await
    }
}

fn text(say: &str, font_size: u16, canvas: Point) {
    draw_text(
        say,
        (canvas.0 as f32) / 2. - get_text_center(say, None, font_size, 1.0, 0.0).x,
        (canvas.1 / 2) as f32,
        font_size as f32,
        WHITE,
    );
}

fn is_keys_down(keys: &[KeyCode]) -> bool {
    keys.iter().any(|&key| is_key_down(key))
}
