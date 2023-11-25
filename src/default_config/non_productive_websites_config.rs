use serde::{Deserialize, Serialize};

use crate::data::{
    website_detection::{DetectionCriteria, DetectionData, DetectionDiscriminant},
    wrappers::WebsiteName,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NonProductiveWebsitesConfiguration {
    pub non_productive_websites: Vec<DetectionData>,
}

impl NonProductiveWebsitesConfiguration {}

impl From<Vec<DetectionData>> for NonProductiveWebsitesConfiguration {
    fn from(non_productive_websites: Vec<DetectionData>) -> Self {
        Self {
            non_productive_websites,
        }
    }
}

impl Default for NonProductiveWebsitesConfiguration {
    fn default() -> Self {
        use DetectionCriteria as Criteria;
        use DetectionDiscriminant as Discriminant;
        use Discriminant::*;
        Self {
            non_productive_websites: vec![
                (
                    WebsiteName::new("Instagram".to_owned()),
                    vec![
                        Criteria::new(EndsWith, "· Direct"),
                        Criteria::new(EndsWith, "Instagram"),
                        Criteria::new(EndsWith, "Instagram photos and videos"),
                    ],
                )
                    .into(),
                (
                    WebsiteName::new("Twitter".to_owned()),
                    vec![
                        Criteria::new(EndsWith, "/ Twitter"),
                        Criteria::new(EndsWith, "/ X"),
                    ],
                )
                    .into(),
                (
                    WebsiteName::new("Whatsapp".to_owned()),
                    vec![
                        Criteria::new(EndsWith, "Whatsapp Web"),
                        Criteria::new(EndsWith, "Whatsapp"),
                    ],
                )
                    .into(),
                (
                    WebsiteName::new("Mediapart".to_owned()),
                    vec![
                        Criteria::new(Contains, "Mediapart"),
                        Criteria::new(EndsWith, "| Mediapart"),
                    ],
                )
                    .into(),
                (
                    WebsiteName::new("Linkedin".to_owned()),
                    vec![Criteria::new(EndsWith, "LinkedIn")],
                )
                    .into(),
                (
                    "Facebook".to_owned(),
                    vec![Criteria::new(EndsWith, "Facebook")],
                )
                    .into(),
                (
                    "Netflix".to_owned(),
                    vec![Criteria::new(EndsWith, "Netflix")],
                )
                    .into(),
                (
                    "Youtube".to_owned(),
                    vec![Criteria::new(EndsWith, "YouTube")],
                )
                    .into(),
                (
                    "OkCupid".to_owned(),
                    vec![Criteria::new(EndsWith, "OkCupid")],
                )
                    .into(),
                ("Tinder".to_owned(), vec![Criteria::new(Contains, "Tinder")]).into(),
                ("Gmail".to_owned(), vec![Criteria::new(EndsWith, "Gmail")]).into(),
                (
                    "Scryfall".to_owned(),
                    vec![Criteria::new(Contains, "Scryfall")],
                )
                    .into(),
                ("EDHREC".to_owned(), vec![Criteria::new(EndsWith, "EDHREC")]).into(),
                (
                    "Moxfield".to_owned(),
                    vec![Criteria::new(Contains, "// Moxfield")],
                )
                    .into(),
                ("Amazon".to_owned(), vec![Criteria::new(Contains, "Amazon")]).into(),
                ("Twitch".to_owned(), vec![Criteria::new(EndsWith, "Twitch")]).into(),
            ],
        }
    }
}
