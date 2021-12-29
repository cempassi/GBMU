use super::consts::TITLE_LEN;
use super::error::Error;
use super::flag::Cgb;

#[derive(Debug, PartialEq, Eq)]
pub enum Title {
    Basic(String),
    Advanced {
        title: String,
        manufacturer: String,
        cgb: Cgb,
    },
}

impl TryFrom<[u8; TITLE_LEN]> for Title {
    type Error = Error;

    fn try_from(raw_title: [u8; TITLE_LEN]) -> Result<Self, Self::Error> {
        if let Ok(cgb) = Cgb::try_from(raw_title[15]) {
            let title = String::from_utf8(raw_title[0..10].into())?
                .trim_end_matches(char::from(0))
                .to_string();
            let manufacturer = String::from_utf8(raw_title[10..14].into())?
                .trim_end_matches(char::from(0))
                .to_string();
            Ok(Title::Advanced {
                title,
                manufacturer,
                cgb,
            })
        } else {
            let title = String::from_utf8(raw_title.into())?
                .trim_end_matches(char::from(0))
                .to_string();
            Ok(Title::Basic(title))
        }
    }
}

impl Title {
    pub fn get(&self) -> String {
        match self {
            Title::Basic(title) => title.clone(),
            Title::Advanced {
                title,
                manufacturer: _,
                cgb: _,
            } => title.clone(),
        }
    }
}
