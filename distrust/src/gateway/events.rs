use num_enum::{FromPrimitive, IntoPrimitive};
use strum_macros::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum ReceivedEvent {
    Dispatch = 0,
    Heartbeat = 1,
    Reconnect = 7,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
    #[num_enum(default)]
    Other,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum SentEvents {
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    RequestGuildMembers = 8,
}

#[derive(Eq, PartialEq, Hash, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayDispatch {
    Ready,
    Resumed,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
}

pub enum Intent {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildBans = 1 << 2,
    GuildEmojisAndStickers = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhooks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessages = 1 << 9,
    GuildMessageReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectMessageReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
    GuildScheduledEvents = 1 << 16,
}