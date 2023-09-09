///# A Discord bot for the most **elistist** mid school in Bratislawa (UwU)
///# What's here:
///- Bryndzové halušky (just kidding, they aren't here)
///- Rust code (obviously)
///- Connections to the Discord and EduPage API through unofficial implementation made by [@loumadev](https://github.com/loumadev)
///
pub mod cmd;
pub mod commands;
pub mod functionality;
pub use crate::functionality::*;
pub use anyhow::Context;
pub use serenity::Client;
pub use std::env;
