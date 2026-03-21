use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

use dptree::case;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Idle,
    Waiting,
    Playing{
        players: Vec<i64>
    },
    End
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Help command")]
    Help,
    #[command(description = "Start a new game")]
    Start,
    #[command(description = "Terminate the game")]
    Kill
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![State::Idle].branch(case![Command::Start].endpoint(start)))
        .branch(case![Command::Kill].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(invalid_state));
    
    /* 
    let callback_query_handler = Update::filter_callback_query().branch(
        case![State::ReceiveProductChoice { full_name }].endpoint(receive_product_selection),
    );*/

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        //.branch(callback_query_handler)
}

async fn start(bot: Bot, dialogue: Dialogue<State, InMemStorage<State>>, msg: Message)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Let's start!").await?;
    dialogue.update(State::Playing { players: vec![1]}).await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: Dialogue<State, InMemStorage<State>>, msg: Message)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.").await?;
    dialogue.update(State::Idle).await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Invalid command: type /help to see the commands available.")
        .await?;
    Ok(())
}