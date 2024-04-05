use bracket_lib::prelude::*;

// The game is either in the menu, playing the game or displaying the game over screen

enum GameMode {
    Menu,
    Playing,
    End
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;


struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5,25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }       
        
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => self.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
            }
        }
    } // close main menu function

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, "(P) Play again?");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
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

// Player struct and implementation below here:

struct Player {
    // x is horizontal co ordinate of the player
    x: i32,
    // y is the vertical position of the player
    y: i32,
    // velocity represents vertical momentum 
    velocity: f32
}


impl Player {
    fn new(x: i32, y:i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // set is a bracket lib function to 'set' a single character on the screen
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            // this function converts a unicode symbol from this source code to the matching Codepage 437 char
            to_cp437('@')
        )
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        // cannot add floating point and integer together, so "as i32" converts to an integer
        self.y += self.velocity as i32;
        // the player only moves the dragon up (or down), incrementing x tracks progress through the level
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // flapping increments the velocity by negative 2 (0 is the top of the screen)
        self.velocity = -2.0
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



