extern crate piston_window;
use piston_window::*;

mod draw;
use draw::*;

mod game;
mod snakes;

fn main() {
    let (block_width, block_height) = (90/2, 60/2);
    let (window_width, window_height) = (to_coord(block_width), to_coord(block_height));
    let mut window: PistonWindow = WindowSettings::new("Snakes!", [window_width, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(block_width, block_height);

    while let Some(event) = window.next() {
        match event {
            Event::Loop(loop_event) => match loop_event {
                Loop::Update(args) => game.update(args.dt),
                Loop::Render(_) => {
                    window.draw_2d(&event, |context, graphics, _device| {
                        clear([0.; 4], graphics);
                        game.render(&context, graphics);
                    });
                }
                _ => (),
            },
            Event::Input(input_event, _) => match input_event {
                Input::Button(args) => match args.button {
                    Button::Keyboard(key) => {
                        if args.state == ButtonState::Press {
                            game.key_down(key);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}
