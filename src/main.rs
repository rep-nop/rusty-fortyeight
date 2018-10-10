// a shitty 2048 clone I wrote in rust instead of studying for calc

// crates
extern crate quicksilver;
extern crate rand;

// modules
mod board;
mod render;

// namespacing
use quicksilver::{
    Result,
    geom::Vector,
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{
        Settings,
        State,
        Window,
        run,
        Event,
    },
};

use board::MoveOpt;

struct GameState {
    board: board::Board,
    sprites: render::SpriteSheet,
    mov_opt: Option<board::MoveOpt>,
}

impl State for GameState {
    // init gamestate
    fn new() -> Result<Self> {
        let mut board = board::Board::new((4, 4));
        let mov_opt = None;
        let sprites = render::SpriteSheet::new((32, 32), (4, 4), "./static/spritesheet2x.png");
        board.starting_tiles();
        
        let gamestate = GameState { board, sprites, mov_opt };
        
        Ok(gamestate)
    }

    // handle input
    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            // arrow keys
            Event::Key(Key::Up, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Up)},
            Event::Key(Key::Down, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Down)},
            Event::Key(Key::Left, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Left)},
            Event::Key(Key::Right, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Right)},

            // wasd
            Event::Key(Key::W, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Up)},
            Event::Key(Key::S, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Down)},
            Event::Key(Key::A, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Left)},
            Event::Key(Key::D, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Right)},
            
            // backspace
            Event::Key(Key::Back, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Undo)},

            // escape
            Event::Key(Key::Escape, ButtonState::Pressed) => {self.mov_opt = Some(MoveOpt::Terminate)},

            _ => {},
        }

        Ok(())
    }

    // react to the handled input
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if self.mov_opt != None {
            self.board.make_move(&self.mov_opt);
            self.mov_opt = None;
        }

        Ok(())
    }

    // draw gamestate to the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        // actually drawing shit here
        self.render_board_bg(window);

        Ok(())
    }
}

// main loop
fn main() {
    // run the game
    run::<GameState>(
        "hey look, it's 2048 but in rust!",
        Vector::new(800, 800),
        Settings::default()
    );
}