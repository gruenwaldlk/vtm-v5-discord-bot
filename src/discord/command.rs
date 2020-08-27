pub(crate) mod meta {
    use serenity::framework::standard::macros::command;
    use serenity::framework::standard::CommandResult;
    use serenity::model::prelude::*;
    use serenity::prelude::*;
    #[command]
    fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
        let _ = msg.reply(ctx, "Pong!");
        Ok(())
    }
    #[command]
    fn uwu(ctx: &mut Context, msg: &Message) -> CommandResult {
        let _ = msg.reply(ctx, "Fuck off!");
        Ok(())
    }
}
