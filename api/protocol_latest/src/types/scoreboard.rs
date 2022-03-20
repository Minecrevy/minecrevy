use minecrevy_io_str::{McRead, McWrite};
use minecrevy_text::Text;

minecrevy_io_str::u8_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum ScoreboardDisplayKind {
        List = 0,
        Sidebar = 1,
        BelowName = 2,
        TeamBlack = 3,
        TeamDarkBlue = 4,
        TeamDarkGreen = 5,
        TeamDarkCyan = 6,
        TeamDarkRed = 7,
        TeamDarkPurple = 8,
        TeamGold = 9,
        TeamGray = 10,
        TeamDarkGray = 11,
        TeamBlue = 12,
        TeamGreen = 13,
        TeamAqua = 14,
        TeamRed = 15,
        TeamLightPurple = 16,
        TeamYellow = 17,
        TeamWhite = 18,
    }
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CreateTeam {
    pub name: Text,
    pub friendly_flags: i8,
    #[options(max_len = 32)]
    pub name_tag_visibility: String,
    #[options(max_len = 32)]
    pub collision_rule: String,
    pub style: TeamStyle,
    pub prefix: Text,
    pub suffix: Text,
    #[options(inner.max_len = 40)]
    pub entities: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct RemoveTeam;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct UpdateTeam {
    pub name: Text,
    pub friendly_flags: i8,
    #[options(max_len = 32)]
    pub name_tag_visibility: String,
    #[options(max_len = 32)]
    pub collision_rule: String,
    pub style: TeamStyle,
    pub prefix: Text,
    pub suffix: Text,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AddTeamEntities {
    #[options(inner.max_len = 40)]
    pub entities: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct RemoveTeamEntities {
    #[options(inner.max_len = 40)]
    pub entities: Vec<String>,
}

minecrevy_io_str::varint_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum TeamStyle {
        Black = 0,
        DarkBlue = 1,
        DarkGreen = 2,
        DarkCyan = 3,
        DarkRed = 4,
        DarkPurple = 5,
        Gold = 6,
        Gray = 7,
        DarkGray = 8,
        Blue = 9,
        Green = 10,
        Aqua = 11,
        Red = 12,
        LightPurple = 13,
        Yellow = 14,
        White = 15,
        Obfuscated = 16,
        Bold = 17,
        Strikethrough = 18,
        Underlined = 19,
        Italic = 20,
        Reset = 21,
    }
}
