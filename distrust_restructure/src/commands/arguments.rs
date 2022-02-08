pub struct Argument<'a> {
    t: ArgumentType<'a>,
}

enum ArgumentType<'a> {
    SubCommand(Vec<Argument<'a>>),
    SubCommandGroup,
    String(String),
    Integer(i64),
    Boolean(bool),
    User(&'a typing::User),
    Channel(&'a typing::Channel),
    Role(/* role type*/),
    Mentionable(/* mentionable trait */),
    Number(f64),
}
