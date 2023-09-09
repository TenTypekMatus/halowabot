use std::env;
use anyhow::Context;
use serenity::Client;
use crate::functionality::Handler;

///# A Discord bot for the most **elistist** mid school in Bratislawa (UwU)
///# What's here:
///- Bryndzové halušky (just kidding, they aren't here)
///- Rust code (obviously)
///- Connections to the Discord and EduPage API through unofficial implementation made by
/// [@loumadev](https://github.com/loumadev)
pub mod commands {
    pub const VIEW_HELP: &'static str = "!pomoc";
    pub const VIEW_MARKS: &'static str = "!pozriznamky";
    pub const VIEW_MSGS: &'static str = "!spravy";
    pub const VIEW_TIMETABLE: &'static str = "!rozvrh";
    pub const VIEW_HWS: &'static str = "!domaceulohy";
    pub const LOGIN: &'static str = "!prihlasit";
}
pub mod msgs {
    pub const HELP_MSG: &'static str = "\
    Discordový robot pre neoficiálny Hálova server.\
    \
    Možnosti:\
    !pomoc                                           Zobraz tieto všetky príkazy\
    !pozriznamky                                     Zobraz známky z EduPage\
    !prihlasit <inštancia> <meno/email> <heslo>      Prihlasiť sa do EduPage\
    !spravy                                          Pozri si svoje správy z EduPage\
    !rozvrh                                          Pozri si svoj rozvrh\
    !domaceulohy                                     Pozri si svoje domáce úlohy\
    ";
}
pub mod functionality {
    use anyhow::anyhow;
    use serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    };
    use crate::msgs::*;
    use crate::commands::*;

    pub struct Handler;

    #[async_trait]
    impl EventHandler for Handler {
        async fn message(&self, ctx: Context, msg: Message) {
            if msg.content == VIEW_HELP {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MSG).await {
                    anyhow!("Error sending message: {:?}", why);
                } else if msg.content == VIEW_MARKS {
                    if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MSG).await {
                        anyhow!("Error sending message: {:?}", why);
                    } else if msg.content == VIEW_MSGS {
                        if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MSG).await {
                            anyhow!("Error sending message: {:?}", why);
                        }
                    } else if msg.content == VIEW_TIMETABLE {
                        if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MSG).await {
                            anyhow!("Error sending message: {:?}", why);
                        }
                    }
                }
            }
            let ready = |_: Context, ready: Ready| {
                println!("{} is connected!", ready.user.name);
            };
        }
    }
}
/// Initialize the bot
pub async fn init_the_bot() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
