use regex::Regex;

pub struct Directory {
    pub season: usize,
}

impl Directory {
    pub fn new(s: &str) -> Option<Self> {
        let regex_with_season = Regex::new(
            r"(?i)^\[.+\]\s(.+)\s([a-z]{0,1})?(?<season>[0-9]{1,2})\s(\(.+\))?(\[.+\])?.{0,}$",
        )
        .unwrap();
        let regex_without_season =
            Regex::new(r"^\[.+\]\s(.+)(\s)?(\(.+\))?(\s\[.+\])?.{0,}$").unwrap();

        let captured = regex_with_season
            .captures(s)
            .or(regex_without_season.captures(s))?;

        let season = captured
            .name("season")
            .and_then(|season| season.as_str().parse::<usize>().ok())
            .unwrap_or(1);

        Some(Self { season })
    }

    pub fn from_normalized(s: &str) -> Option<Self> {
        let season = s.replace("Season", "").trim().parse::<usize>().ok()?;

        Some(Self { season })
    }
}
