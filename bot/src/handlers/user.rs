use super::internal::user as internal;
use crate::{commands, commands::UserCommand};
use teloxide::{
    adaptors::DefaultParseMode, prelude::Requester, requests::ResponseResult, types::Message, Bot,
};

pub async fn handle(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    cmd: UserCommand,
) -> ResponseResult<()> {
    let text = match cmd {
        UserCommand::AlreadyMaxed => internal::generate_search_string(true).await,
        UserCommand::NonMaxed => internal::generate_search_string(false).await,

        UserCommand::Help => commands::help(),

        UserCommand::Version => {
            format!(
                "🤖 I was built with commit: _{}_",
                internal::get_commit_hash()
            )
        }
    };

    let _ = bot.send_message(msg.chat.id, text).await;

    Ok(())
}