use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub fn multiply(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let product = one * two;

    let _ = msg.channel_id.say(&ctx.http, product);

    Ok(())
}

#[command]
#[aliases("sum")]
pub fn add(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Yeah.... I need some numbers.");
    } else {
        let one = args.single::<f64>().unwrap();
        let two = args.single::<f64>().unwrap();
        let sum = one + two;
        let _ = msg.channel_id.say(&ctx.http, sum);
    }

    Ok(())
}
