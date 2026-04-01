use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

use dptree::case;

use crate::c_ffi;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Idle,
    Waiting,
    Playing{ players: Vec<i64>},
    ReceiveGuess,
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

    //filtra i comandi ("/command")
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![State::Idle].branch(case![Command::Start].endpoint(start)))
        .branch(case![Command::Kill].endpoint(cancel));

    //filtra i messaggi
    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveGuess]).endpoint(receive_guess)
        .branch(dptree::endpoint(invalid_state));
/* 
    let callback_query_handler = Update::filter_callback_query().branch(
        case![State::ReceiveGuess { guess }].endpoint(receive_guess),
    );*/

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
    //    .branch(callback_query_handler)
}

async fn start(bot: Bot, dialogue: Dialogue<State, InMemStorage<State>>, msg: Message)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Let's start!").await?;
    unsafe {
        c_ffi::set_paths();
        println!("Paths setup");
        c_ffi::setup_game();
        println!("Game setup");
        c_ffi::start_new_game();
        println!("Game started");
    }
    
    dialogue.update(State::ReceiveGuess).await?;
    bot.send_message(msg.chat.id, "Enter guess or command: ").await?;
    Ok(())
}

async fn receive_guess(bot: Bot, dialogue: Dialogue<State, InMemStorage<State>>,  msg: Message)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = msg.text()  // Option<&str>
                                    .map(|text| c_ffi::play_turn_wrapper(text))
                                    .unwrap_or_else(|| "Messaggio non valido".to_string());
    println!("Risultato: {:}", result);
    bot.send_message(msg.chat.id, result).await?;

    if unsafe { c_ffi::is_game_ended() } {
        dialogue.update(State::Idle).await?;
        let attempts = unsafe { c_ffi::get_attempt_number() };
        let msg_string = format!("Congratulations! You won in {} attempts!", attempts);
        bot.send_message(msg.chat.id, msg_string).await?;
    }

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