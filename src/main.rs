use poise::serenity_prelude as serenity;
pub use poise::{serenity_prelude::GatewayIntents, PrefixFrameworkOptions};
use std::time::SystemTime;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let t1 = SystemTime::now();
    let msg = ctx.say("Pong!").await?;
    let ping = t1.elapsed().expect("..").as_millis();
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    msg.edit(ctx, |m| m.content(format!("Pong! {}ms", ping)))
        .await?;
        Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let discord_token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), ping()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("+".to_string()),
                mention_as_prefix: true,
                case_insensitive_commands: false,
                ignore_bots: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}