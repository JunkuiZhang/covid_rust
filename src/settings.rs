// Settings...
pub const POP_NUM: u64 = 150;
pub const WINDOW_WIDTH: u32 = 600;
pub const INFECT_RADIUS: f64 = 0.03 * WINDOW_WIDTH as f64;
pub const INITIAL_INFECTED_PARTION: f64 = 0.1;
pub const INITIAL_AWARE_PARTION: f64 = 0.9;
pub const INFECTION_CHANCE: f64 = 0.02;
pub const AWARE_ENTITY_MOVE_SPPED: f64 = 10.0;
pub const NOT_AWARE_ENTITY_MOVE_MUL: f64 = 5.0;
pub const AWARE_RADIUS_MUL: f64 = 2.0;
pub const AWARE_RADIUS: f64 = INFECT_RADIUS * AWARE_RADIUS_MUL;
