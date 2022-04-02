use minecrevy_util::ticks::Ticks;

use crate::Text;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Title {
    pub title: Option<Text>,
    pub subtitle: Option<Text>,
    pub times: Option<TitleTimes>,
}

impl Title {
    pub fn new(title: Option<Text>, subtitle: Option<Text>, times: Option<TitleTimes>) -> Self {
        Self {
            title,
            subtitle,
            times,
        }
    }

    pub fn title(title: Text, times: Option<TitleTimes>) -> Self {
        Self {
            title: Some(title),
            subtitle: None,
            times,
        }
    }

    pub fn subtitle(subtitle: Text, times: Option<TitleTimes>) -> Self {
        Self {
            title: None,
            subtitle: Some(subtitle),
            times,
        }
    }
}

impl From<Text> for Title {
    #[inline]
    fn from(text: Text) -> Self {
        Self::title(text, None)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct TitleTimes {
    pub fade_in: Ticks,
    pub stay: Ticks,
    pub fade_out: Ticks,
}
