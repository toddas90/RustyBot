// Copyright 2020 Andrew Todd

// This file is part of RustyBot.

// RustyBot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.

// RustyBot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with RustyBot.  If not, see <https://www.gnu.org/licenses/>.

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn math(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = meval::eval_str(args.message().to_string()).unwrap();
    msg.channel_id.say(&ctx.http, r).await?;

    Ok(())
}