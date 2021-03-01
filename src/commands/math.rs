use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn math(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let r = meval::eval_str(args.message().to_string()).unwrap();
    msg.channel_id.say(&ctx.http, r).await?;

    Ok(())
}