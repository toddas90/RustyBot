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

use serenity::{futures::AsyncReadExt, prelude::*};
use serenity::model::prelude::*;
use serenity::framework::standard::{ 
    CommandResult,
    macros::command,
};
use rand::{thread_rng, Rng};
use webpage::{Webpage, WebpageOptions};

const HELP_MESSAGE: &str = "
```Hello there, Human!

You have summoned me. Let's see about getting you what you need.

=> !#help
          
=> !#math [expression]

=> !#coinflip

=> !#diceroll

=> !#crypto [currency] // Broken AF

=> !#quit
          
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

#[command]
async fn crypto(ctx: &Context, msg: &Message) -> CommandResult {
    let url: String = String::from("http://rate.sx/") + &msg.content;
    let info = Webpage::from_url(&url, WebpageOptions::default())
        .expect("Could not read from URL");
    msg.reply(&ctx.http, &info.html.text_content).await?; // Too long to print :(

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