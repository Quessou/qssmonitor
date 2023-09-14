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
                    WebsiteName {
                        data: "Instagram".to_owned(),
                    },
                    vec![
                        Criteria::new(EndsWith, "· Direct"),
                        Criteria::new(EndsWith, "Instagram"),
                        Criteria::new(EndsWith, "Instagram photos and videos"),
                    ],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Twitter".to_owned(),
                    },
                    vec![
                        Criteria::new(EndsWith, "/ Twitter"),
                        Criteria::new(EndsWith, "/ X"),
                    ],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Whatsapp".to_owned(),
                    },
                    vec![
                        Criteria::new(EndsWith, "Whatsapp Web"),
                        Criteria::new(EndsWith, "Whatsapp"),
                    ],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Mediapart".to_owned(),
                    },
                    vec![
                        Criteria::new(Contains, "Mediapart"),
                        Criteria::new(EndsWith, "| Mediapart"),
                    ],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Linkedin".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "LinkedIn")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Facebook".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "Facebook")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Netflix".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "Netflix")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Youtube".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "YouTube")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "OkCupid".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "OkCupid")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Tinder".to_owned(),
                    },
                    vec![Criteria::new(Contains, "Tinder")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Gmail".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "Gmail")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Scryfall".to_owned(),
                    },
                    vec![Criteria::new(Contains, "Scryfall")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "EDHREC".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "EDHREC")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Moxfield".to_owned(),
                    },
                    vec![Criteria::new(Contains, "// Moxfield")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Amazon".to_owned(),
                    },
                    vec![Criteria::new(Contains, "Amazon")],
                )
                    .into(),
                (
                    WebsiteName {
                        data: "Twitch".to_owned(),
                    },
                    vec![Criteria::new(EndsWith, "Twitch")],
                )
                    .into(),
            ],
        }
    }
}
