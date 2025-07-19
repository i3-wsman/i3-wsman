#[macro_use]
extern crate lazy_static;

use i3_ipc::{Connect, I3Stream, I3 as I3_api};
use std::collections::HashMap;

pub mod commands;
pub mod common;
pub mod config;
pub mod groups;
pub mod i3;
pub mod polybar;
pub mod state;

lazy_static! {
    pub static ref CONFIG: config::global::Config = config::global::load_cfg();
    pub static ref POLYBAR_CFG: config::polybar::Config = Default::default();
    pub static ref I3: I3Stream = I3_api::connect().unwrap();
}

pub type CommandFn = fn(Vec<String>);
pub type Commands = HashMap<&'static str, CommandFn>;
pub type CommandMap = HashMap<&'static str, Commands>;

pub static DEFAULT_CMD: &str = "";
pub static HELP_CMD: &str = "help";
pub static WILD_CMD: &str = "*";
