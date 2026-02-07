use crate::handler::command_handler::CommandHandler;

pub fn register_global_commands(command_handler: &mut CommandHandler,) {
    command_handler.register(String::from("q"), "@", |mos, _args| {
        mos.state_handler.should_quit = true;
        Ok(String::from("Quit command executed"))
    });
}