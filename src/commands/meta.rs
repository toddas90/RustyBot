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
use webpage::{Webpage, WebpageOptions};

const HELP_MESSAGE: &str = "
```Hello there, Human!

You have summoned me. Let's see about getting you what you need.

=> !!help [command]
          
=> !!math [expression]

=> !!coinflip

=> !!diceroll

=> !!crypto [currency] // Broken AF

=> !!quit
          
I hope that resolves your issue!
-- Ferris```   
";

const HELP_MESSAGE_MATH: &str = "
```meval supports basic mathematical operations on floating point numbers:

binary operators: +, -, *, /, % (remainder), ^ (power)
unary operators: +, -

functions implemented using functions of the same name in Rust std library:

sqrt, abs
exp, ln
sin, cos, tan, asin, acos, atan, atan2
sinh, cosh, tanh, asinh, acosh, atanh
floor, ceil, round
signum
other functions:

max(x, ...), min(x, ...): maximum and minimumum of 1 or more numbers
constants:

pi
e

Usage:: !!math [expression]```
";

const HELP_MESSAGE_COIN: &str = "
```Flips a coin!
Usage: !!coinflip```
";

const HELP_MESSAGE_DICE: &str = "
```Rolls a single die!
Usage: !!diceroll```
";

const HELP_MESSAGE_CRYPTO: &str = "
```Queries rate.sx for the latest crypto exchange prices.
Get general info: !!crypto
Get specific info: !!crypto [coin name]```
";

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message, arg: Args) -> CommandResult {
    match arg.message() {
        "coinflip" => msg.reply(&ctx.http, HELP_MESSAGE_COIN).await?,
        "diceroll" => msg.reply(&ctx.http, HELP_MESSAGE_DICE).await?,
        "math" => msg.reply(&ctx.http, HELP_MESSAGE_MATH).await?,
        "crypto" => msg.reply(&ctx.http, HELP_MESSAGE_CRYPTO).await?,
        _ => msg.reply(&ctx.http, HELP_MESSAGE).await?,
    };

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