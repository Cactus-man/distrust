use super::SlashCommand;

pub struct SlashCommandBuilder {
    name: String,
}

impl SlashCommandBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }

    pub fn add_argument(self) -> SlashCommandBuilder {
        todo!()
    }

    pub fn build(&mut self) -> SlashCommand {
        todo!()
    }
}
