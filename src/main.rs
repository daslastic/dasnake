use macroquad::prelude::*;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 320;
const INIT_LENGTH: usize = 3;
fn window_conf() -> Conf {
    Conf {
        window_title: "dasnake".to_owned(),
        fullscreen: false,
        sample_count: 0,
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

enum State {
    Menu,
    Playing,
    Lose,
    Win,
}

struct Snake {
    pos: IVec2,
    last_pos: Vec<IVec2>,
    len: usize,
    color: Color,
    next: IVec2,
    last_key: Option<KeyCode>,
    super_speed: bool,
}
impl Snake {
    pub fn new(pos: IVec2, color: Color) -> Self {
        Self {
            pos,
            last_pos: Vec::new(),
            len: 3,
            color,
            next: IVec2::new(1, 0),
            last_key: None,
            super_speed: false,
        }
    }
}

impl Snake {
    pub fn last_pressed(&self, key: KeyCode) -> bool {
        self.last_key == Some(key)
    }
}

struct Game {
    canvas: IVec2,
    canvas_size: IVec2,
    tile_size: i32,
    apples: Vec<IVec2>,
    speed: f64,
    state: State,
}

impl Game {
    pub fn new(canvas: IVec2, tile_size: i32, speed: f64) -> Self {
        let canvas_size = IVec2::new(canvas.x / tile_size, canvas.y / tile_size);
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
        let pos = IVec2::new(
            rand::gen_range(0, self.canvas_size.x),
            rand::gen_range(0, self.canvas_size.y),
        );

        // if entire screen is full -- small possibility this is wrong lol
        if snake.last_pos.len() > (self.canvas_size.x * self.canvas_size.y - 1) as usize {
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
        snake.len = INIT_LENGTH;
        self.apples.clear();
        self.spawn_apple(snake);
        snake.pos = IVec2::ZERO;
        snake.last_pos.clear();
        snake.next = IVec2::new(1, 0);
        snake.last_key = None;
        self.state = State::Playing;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(IVec2::new(WIDTH, HEIGHT), 20, 0.04);
    let mut snake = Snake::new(IVec2::ZERO, GREEN);
    let mut last_update = get_time();

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
                // render
                {
                    // player
                    draw_rectangle(
                        (game.tile_size * snake.pos.x) as f32,
                        (game.tile_size * snake.pos.y) as f32,
                        game.tile_size as f32,
                        game.tile_size as f32,
                        snake.color,
                    );

                    // player tail
                    for pos in snake.last_pos.iter() {
                        draw_rectangle(
                            (game.tile_size * pos.x) as f32,
                            (game.tile_size * pos.y) as f32,
                            game.tile_size as f32,
                            game.tile_size as f32,
                            snake.color,
                        );
                    }

                    // apples
                    for pos in game.apples.iter() {
                        draw_rectangle(
                            (game.tile_size * pos.x) as f32,
                            (game.tile_size * pos.y) as f32,
                            game.tile_size as f32,
                            game.tile_size as f32,
                            RED,
                        );
                    }
                }

                // update snake
                if elapsed >= game.speed || (snake.super_speed && elapsed >= game.speed / 2.) {
                    snake.pos += snake.next;

                    // lose
                    if snake.last_pos.contains(&snake.pos) {
                        game.state = State::Lose;
                        continue;
                    }

                    // out of bounds
                    snake.pos.x = (snake.pos.x + game.canvas_size.x) % game.canvas_size.x;
                    snake.pos.y = (snake.pos.y + game.canvas_size.y) % game.canvas_size.y;

                    last_update = get_time();

                    // increase length
                    {
                        let mut i = 0;
                        snake.last_pos.insert(0, snake.pos);
                        snake.last_pos.retain(|_| {
                            i += 1;
                            i < snake.len
                        })
                    }

                    // apple collision
                    game.apples.retain(|apple_pos| {
                        if apple_pos.eq(&snake.pos) {
                            snake.len += 1;
                            false
                        } else {
                            true
                        }
                    });

                    if game.apples.len() == 0 {
                        game.spawn_apple(&snake);
                    }
                }

                // input snake
                if is_key_down(KeyCode::W) && !snake.last_pressed(KeyCode::S) {
                    snake.next = IVec2::new(0, -1);
                    snake.last_key = Some(KeyCode::W);
                } else if is_key_down(KeyCode::S) && !snake.last_pressed(KeyCode::W) {
                    snake.next = IVec2::new(0, 1);
                    snake.last_key = Some(KeyCode::S);
                } else if is_key_down(KeyCode::A) && !snake.last_pressed(KeyCode::D) {
                    snake.next = IVec2::new(-1, 0);
                    snake.last_key = Some(KeyCode::A);
                } else if is_key_down(KeyCode::D) && !snake.last_pressed(KeyCode::A) {
                    snake.next = IVec2::new(1, 0);
                    snake.last_key = Some(KeyCode::D);
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

pub fn text(say: &str, font_size: u16, canvas: IVec2) {
    draw_text(
        say,
        (canvas.x as f32) / 2. - get_text_center(say, None, font_size, 1.0, 0.0).x,
        (canvas.y / 2) as f32,
        font_size as f32,
        WHITE,
    );
}
