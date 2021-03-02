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

use rand::{thread_rng, Rng};
use serenity::prelude::*;
use serenity::model::prelude::*;
use tracing::error;
use serenity::framework::standard::{ 
    CommandResult,
    macros::command,
};

#[command]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    if coinflip_helper() {
        if let Err(why) = msg.reply(&ctx.http, "Heads").await {
            error!("Error sending message: {:?}", why);
        }
    } else {
        if let Err(why) = msg.reply(&ctx.http, "Tails").await {
            error!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}

#[command]
async fn diceroll(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(&ctx.http, diceroll_helper()).await {
        error!("Error sending message: {:?}", why);
    }

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