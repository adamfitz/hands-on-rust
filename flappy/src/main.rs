use bracket_lib::prelude::*;

// The gmae is either in the menu, playing the game or displaying the game over screen
enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu
        };
    }

    fn play(&mut self, ctx: &mut Bterm){
        //TODO: Fillin this stub later
        self.mode = GameMode::End;
        }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        }

        if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => self.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
            }
        }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, "(P) Play again?");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.guitting = true,
                _ => {}
            }
        }
    }
}

// implement the GameState trait (from bracket-lib) on the State struct created above
// this is similar to implementing functions for the struct
impl GameState for State {
    // GameState requires implementation of the tick function with this signature
    // &mut self allows tick function to access State instance and modify it
    // ctx provides a window to the currently running bracket-terminal to get input information from the player and for
    // sending commands to draw to the window.
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}

fn main() -> BError {
    // request an 80x50 pixel terminal
    let context = BTermBuilder::simple80x50()
        // set the window title
        .with_title("Flappy Dragon")
        // trigger the object build/creation
        .build()?; // the ? allows errors to be passed to the parent (the main function)
    
    // start executing the game loop and link the engine with the State struct
    main_loop(context, State::new())
}



