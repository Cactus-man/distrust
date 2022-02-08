mod arguments;
mod builder;

pub use self::builder::SlashCommandBuilder;

use self::arguments::Argument;
use typing::{Guild, Snowflake};

pub struct SlashCommand<'a> {
    id: Snowflake,
    t: AppCommandType,
    app: Snowflake,
    guild: Option<&'a Guild>,
    name: String,
    desc: String,
    options: Option<Vec<Argument<'a>>>,
    default_permission: Option<bool>,
    version: Snowflake,
}

pub enum AppCommandType {
    Chat,
    User,
    Message,
}

impl SlashCommand<'_> {
    fn name(&self) -> &str {
        &self.name
    }

    fn delete(&mut self) {
        todo!()
    }
}
