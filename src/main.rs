use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
};

pub mod commands;
use commands::{schema, State};
pub mod c_ffi;

#[tokio::main]
async fn main() {
   match dotenvy::dotenv() {
      Ok(path) => println!("Token retrieved."),
      Err(e) => println!("Token not found.")
   }

   println!("Starting command bot...");
   let bot = Bot::from_env();

   Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
