use bracket_lib::prelude::*;

struct State {}

// implement the GameState trait (from bracket-lib) on the State struct created above
// this is similar to implementing functions for the struct
impl GameState for State {
    // GameState requires implementation of the tick function with this signature
    // &mut self allows tick function to access State instance and modify it
    // ctx provides a window to the currently running bracket-terminal to get input information from the player and for
    // sending commands to draw to the window.
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx stands for context and provides functions to interact with the game display
        ctx.cls(); // clears the game window (screen)
        // allows printing of text to the game window, 1,1 are screen space coordinates
        // print accespts a String (unlike println!)
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    // request an 80x50 pixel terminal
    let context = BTermBuilder::simple100x80()
        // set the window title
        .with_title("Flappy Dragon")
        // trigger the object build/creation
        .build()?;
    
    // start executing the game loop and link the engine with the State struct
    main_loop(context, State{})
}



