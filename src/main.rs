use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_COMMAND: &str = "!help";

struct Handler;

#[tokio::main]