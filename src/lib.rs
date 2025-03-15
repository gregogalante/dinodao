use wasm_bindgen::prelude::*;
use web_sys::window;
use std::cell::RefCell;
use std::rc::Rc;

struct GameState {
    active: bool,
    token: String,
    game_width: u32,
    last_trigger: f64,
    last_jump: f64,
    speed: f64,
    points: u32,
}

thread_local! {
    static GAME_STATE: RefCell<GameState> = RefCell::new(GameState {
        active: false,
        token: String::new(),
        game_width: 0,
        last_trigger: 0.0,
        last_jump: 0.0,
        speed: 5.0,
        points: 0,
    });
}

fn get_current_time() -> f64 {
    window()
        .expect("no global window")
        .performance()
        .expect("no performance object")
        .now()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("no global window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame`");
}

fn setup_game_loop() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    const USER_PERCENTAGE_POSITION_START: f64 = 57.5;
    const USER_PERCENTAGE_POSITION_END: f64 = 62.5;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let current_time = get_current_time();

        // Update the game state
        GAME_STATE.with(|state| {
            let mut state = state.borrow_mut();
            if state.active {
                // Calculate the lifetime of the jump based on the game width
                let jump_lifetime = (state.game_width as f64) * 1.0;
                // Calculate the lifetime of the trigger based on the game width
                let trigger_lifetime = (state.game_width as f64) * state.speed;

                // Trigger the game if the last_trigger is 0
                if state.last_trigger == 0.0 {
                    state.last_trigger = current_time;
                    dinodaoTrigger(true);
                    return;
                }

                // Check if the game is over
                let trigger_percentage = ((current_time - state.last_trigger) / trigger_lifetime) * 100.0;
                if state.last_jump == 0.0 {
                    if trigger_percentage >= USER_PERCENTAGE_POSITION_START && trigger_percentage <= USER_PERCENTAGE_POSITION_END {
                        state.active = false;

                        // TODO: Here we should encrypt the result of the game and send it back crypted as a new token.

                        dinodaoEnd(state.token.clone());
                        return;
                    }
                }
                
                // Reset the jump if the last_jump is not 0 for more than jump_lifetime
                if state.last_jump != 0.0 && current_time - state.last_jump > jump_lifetime {
                    state.last_jump = 0.0;
                    dinodaoJump(false);
                }

                // Reset the trigger if the last_trigger is not 0 for more than trigger_lifetime and update points and speed
                if current_time - state.last_trigger > trigger_lifetime {
                    state.last_trigger = 0.0;
                    dinodaoTrigger(false);

                    state.points += 10;
                    dinodaoUpdatePoints(state.points);

                    if state.points > 0 && (state.points % 50) == 0 && state.speed > 1.0 {
                        state.speed -= 0.5;
                        dinodaoUpdateSpeed(state.speed);
                    }
                }
            }
        });

        // Request the next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

// EXPOSED FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen]
extern "C" {
    fn dinodaoTrigger(status: bool);
    fn dinodaoJump(status: bool);
    fn dinodaoUpdateSpeed(speed: f64);
    fn dinodaoUpdatePoints(points: u32);
    fn dinodaoEnd(token: String);

    // Use console.log to print the message
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn play(token: &str, game_width: u32) -> bool {
    // Token should be a string of 32 characters
    if token.len() != 32 {
        return false;
    }

    // Game width should be greater than 100
    if game_width < 100 {
        return false;
    }

    // Set the game state by storing the token and setting the game as active on the thread local storage
    GAME_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.active = true;
        state.token = token.to_string();
        state.game_width = game_width;
        state.last_trigger = 0.0;
        state.last_jump = 0.0;
        state.speed = 5.0;
        state.points = 0;
    });

    // Set up the game loop
    setup_game_loop();

    true
}

#[wasm_bindgen]
pub fn jump() {
    GAME_STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        if state.last_jump == 0.0 {
            state.last_jump = get_current_time();
            dinodaoJump(true);
        }
    });
}
