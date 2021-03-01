use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use rand::{thread_rng, Rng};

const HELP_MESSAGE: &str = "
```Hello there, Human!

You have summoned me. Let's see about getting you what you need.

=> $$help
          
=> $$math [expression]

=> $$coinflip

=> $$diceroll

=> $$quit
          
I hope that resolves your issue!
-- Ferris```
          
";

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}

#[command]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    if coinflip_helper() {
        msg.reply(&ctx.http, "Heads").await?;

    } else {
        msg.reply(&ctx.http, "Tails").await?;
    }

    Ok(())
}

#[command]
async fn diceroll(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, diceroll_helper()).await?;

    Ok(())
}

fn coinflip_helper() -> bool {
    let mut rng = thread_rng();
    rng.gen_bool(1.0 / 2.0)
}

fn diceroll_helper() -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(1..7)
}