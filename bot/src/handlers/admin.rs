use super::internal::admin as internal;
use crate::{commands::AdminCommand, utils};
use teloxide::{
    adaptors::DefaultParseMode, prelude::Requester, requests::ResponseResult, types::Message, Bot,
};

pub async fn handle(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    cmd: AdminCommand,
) -> ResponseResult<()> {
    // Permission guard clause
    if let Some(x) = utils::check_permission(&msg) {
        if !x {
            let _ = bot
                .send_message(msg.chat.id, "You are not allowed to do that")
                .await;

            log::warn!(
                "{} tried using an admin command",
                msg.from().map_or("<no message WTF>", |from| from.username.as_deref().unwrap_or("<no username>"))
            );

            return Ok(());
        }
    } else {
        log::error!("Could't check permission...");
        return Ok(());
    }

    let text = match cmd {
        AdminCommand::Add(name) => internal::update_pokemon(name, 1, "level40").await,
        AdminCommand::Dec(name) => internal::update_pokemon(name, -1, "level40").await,

        AdminCommand::Catch(name) => internal::update_pokemon(name, 1, "tradeable").await,
        AdminCommand::Trade(name) => internal::update_pokemon(name, -1, "tradeable").await,
    };

    let _ = bot.send_message(msg.chat.id, text).await;

    Ok(())
}
