///# A Discord bot for the most **elistist** mid school in Bratislawa (UwU)
///# What's here:
///- Bryndzové halušky (just kidding, they aren't here)
///- Rust code (obviously)
///- Connections to the Discord and EduPage API through unofficial implementation made by
/// [@loumadev](https://github.com/loumadev)
pub mod commands {
    pub const HELP_MESSAGE: &'static str = "!pomoc";
    pub const VIEW_MARKS: &'static str = "!pozriznamky";
    pub const VIEW_MSGS: &'static str = "!spravy";
    pub const HELP_TIMETABLE: &'static str = "!rozvrh";
    pub const HELP_HWS: &'static str = "!domaceulohy";
}
pub mod functionality {
    #[async_trait]
    impl EventHandler for Handler {
        async fn message(&self, ctx: Context, msg: Message) {
            if msg.content == HELP_COMMAND {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        /// Some OOP
        async fn ready(&self, _: Context, ready: Ready) {
            println!("{} is connected!", ready.user.name);
        }
    }
}
/// Initialize the bot
async fn init_the_bot() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
