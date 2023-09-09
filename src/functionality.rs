pub mod functionality {
    use anyhow::anyhow;
    use serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    };
    use cmd::cmdfunctionality::*;
    use crate::{cmd, commands};
    use commands::commands::*;
    use std::env;
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
                        } else if msg.content == VIEW_HWS {
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
        } // we need to move the other commands above this line
    }
}
