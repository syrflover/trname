use regex::Regex;

pub struct File {
    pub episode: usize,
    pub ext: String,
}

impl File {
    pub fn new(title: &str, s: &str) -> Option<Self> {
        fn parse_episode(s: &str) -> Option<usize> {
            if s.starts_with('0') {
                return parse_episode(&s[1..]);
            } else {
                &s
            }
            .parse()
            .ok()
        }

        let s = s.trim();

        for (cond, regex) in [
            (
                // SxxExx
                s.starts_with(title),
                r"(?i)^.*S[0-9]{2,2}E(?<episode>[0-9]{2,2})\.(?<ext>\w+)$",
            ),
            (
                // SubsPlease
                s.starts_with("[SubsPlease]"),
                r"(?i)^\[SubsPlease\] .+ - (?<episode>[0-9]{1,2})(\s)?(\((480p|720p|1080p)\))?(\s)?(\[\w+\])?.*\.(?<ext>\w+)$",
            ),
            (
                // Moozzi2
                s.starts_with("[Moozzi2]"),
                r"(?i)^\[Moozzi2\] .+ - (?<episode>[0-9]{1,2}).+\.(?<ext>\w+)$",
            ),
            (
                // Ioroid
                s.starts_with("[Ioroid]"),
                r"(?i)^\[Ioroid\] .+ - (?<episode>[0-9]{1,2})(\s)?(\(.+\))?(\s)?(\[.+\])?.*\.(?<ext>\w+)$",
            ),
        ] {
            if cond {
                let captured = Regex::new(regex).unwrap().captures(s);

                if let Some(captured) = captured {
                    let episode = parse_episode(&captured["episode"])?;
                    let ext = captured["ext"].to_owned();

                    return Some(Self { episode, ext });
                }
            }
        }

        let regex_with_rel_name_1 = Regex::new(
            r"(?i)^\[.+\]\s(.+)\s(?<episode>[0-9]+)(\s[a-z]+)?(\s\(.+\))?(\s\[.+\])?\w*\.(?<ext>\w+)$",
        )
        .unwrap();

        // let regex_without_rel_name_4 = Regex::new(
        //     r"(?i)^.+\s(.+)\s(?<episode>[0-9]+)(\s[a-z]+)?(\s\(.+\))?(\s\[.+\])?.+\.(?<ext>\w+)$",
        // )
        // .unwrap();
        // let regex_without_rel_name_3 =
        //     Regex::new(r"^.*(?<episode>[0-9]{3,3}).*\.(?<ext>\w+)$").unwrap();
        let regex_without_rel_name_2 =
            Regex::new(r"^.*(?<episode>[0-9]{2,2}).*\.(?<ext>\w+)$").unwrap();
        let regex_without_rel_name_1 =
            Regex::new(r"^.*(?<episode>[0-9]{1,1}).*\.(?<ext>\w+)$").unwrap();

        let captured = regex_with_rel_name_1
            .captures(s)
            // .or(regex_without_rel_name_4.captures(s))
            // .or(regex_without_rel_name_3.captures(s))
            .or(regex_without_rel_name_2.captures(s))
            .or(regex_without_rel_name_1.captures(s))?;

        let episode = parse_episode(&captured["episode"])?;

        let ext = captured["ext"].to_owned();

        Some(Self { episode, ext })
    }
}

#[test]
fn tests_regex() {
    use crate::directory::Directory;

    //
    // File
    //

    let tvdb = File::new("Tensui no Sakuna-hime", "Tensui no Sakuna-hime S01E03.smi").unwrap();

    assert_eq!(tvdb.episode, 3);

    let beatrice = File::new(
        "Kono Subarashii Sekai ni Shukufuku wo!",
        "[Beatrice-Raws] Kono Subarashii Sekai ni Shukufuku wo! 04 (BDRip 1920x1080 x264 FLAC).mkv",
    )
    .unwrap();

    assert_eq!(beatrice.episode, 4);

    let beatrice = File::new(
        "Kaguya-sama wa Kokurasetai",
        "[Beatrice-Raws] Kaguya-sama wa Kokurasetai 08 [BDRip 1920x1080 HEVC FLAC]_rev.mkv",
    )
    .unwrap();

    assert_eq!(beatrice.episode, 8);

    let subsplease = File::new(
        "Kono Subarashii Sekai ni Bakuen wo!",
        "[SubsPlease] Kono Subarashii Sekai ni Bakuen wo! - 01 (1080p) [10709A2B].mkv",
    )
    .unwrap();

    assert_eq!(subsplease.episode, 1);

    let subsplease = File::new(
        "Tensui no Sakuna-hime",
        "[SubsPlease] Tensui no Sakuna-hime - 01 (1080p) [F0D958CB]-2.smi",
    )
    .unwrap();

    assert_eq!(subsplease.episode, 1);

    let subsplease = File::new(
        "Tensui no Sakuna-hime",
        "[SubsPlease] Tensui no Sakuna-hime - 01 [F0D958CB]-2.smi",
    )
    .unwrap();

    assert_eq!(subsplease.episode, 1);

    let subsplease =
        File::new("", "[SubsPlease] Tensui no Sakuna-hime - 01 (1080p)-2.smi").unwrap();

    assert_eq!(subsplease.episode, 1);

    let moozzi2 = File::new(
        "Fullmetal Alchemist Brotherhood",
        "[Moozzi2] Fullmetal Alchemist Brotherhood - 64 END (BD 1920x1080 x.264 Flac).mkv",
    )
    .unwrap();

    assert_eq!(moozzi2.episode, 64);

    let moozzi2 = File::new(
        "Spy x Family",
        "[Moozzi2] Spy x Family S2 - 12 END [ 37 ] (BD 1920x1080 x265-10Bit Flac).mkv",
    )
    .unwrap();

    assert_eq!(moozzi2.episode, 12);

    let moozzi2 = File::new(
        "Watashi no Shiawase na Kekkon",
        "[Moozzi2] Watashi no Shiawase na Kekkon - 10 (BD 1920x1080 x265-10Bit Flac).mkv",
    )
    .unwrap();

    assert_eq!(moozzi2.episode, 10);

    let ioroid = File::new(
        "Shikanoko Nokonoko Koshitantan",
        "[Ioroid] Shikanoko Nokonoko Koshitantan - 02 [ABEMA WEB-DL 1080p AVC AAC]-2.smi",
    )
    .unwrap();

    assert_eq!(ioroid.episode, 2);

    let ioroid = File::new(
        "Urusei Yatsura",
        "[Ioroid] Urusei Yatsura (2022) 2nd Season - 23 (46) [AMZN WEB-DL 1080p AVC E-AC3].mkv",
    )
    .unwrap();

    assert_eq!(ioroid.episode, 23);

    let without_rel_name = File::new(
        "Kono Subarashii Sekai ni Bakuen wo!",
        "Kono Subarashii Sekai ni Bakuen wo! - 01LoremIpsum.smi",
    )
    .unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = File::new("Sousou no Frieren", "프리렌 1.ass").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = File::new("Sousou no Frieren", "프리렌 1 (F).ass").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = File::new("No Game No Life", "nogame01.ass").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = File::new("No Game No Life", "nogame12.ass").unwrap();

    assert_eq!(without_rel_name.episode, 12);

    let without_rel_name = File::new("No Game No Life", "no1game12.ass").unwrap();

    assert_eq!(without_rel_name.episode, 12);

    let without_rel_name = File::new("Bocchi the Rock!", "Bocchi the Rock! S01E05.ass").unwrap();

    assert_eq!(without_rel_name.episode, 5);

    let without_rel_name = File::new("Bocchi the Rock!", "5.ass").unwrap();

    assert_eq!(without_rel_name.episode, 5);

    let without_rel_name = File::new(
        "Spice and Wolf (2024)",
        "Ookami to Koushinryou (2024) - 01SubsPlease.smi",
    )
    .unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name =
        File::new("Spice and Wolf (2024)", "Spice and Wolf (2024) S01E04.smi").unwrap();

    assert_eq!(without_rel_name.episode, 4);

    let without_rel_name = File::new(
        "Kaguya-sama wa Kokurasetai",
        "카구야 님은 고백받고 싶어 03.smi",
    )
    .unwrap();

    assert_eq!(without_rel_name.episode, 3);

    let without_rel_name = File::new(
        "Kaguya-sama wa Kokurasetai",
        "카구야 님은 고백받고 싶어 11.smi",
    )
    .unwrap();

    assert_eq!(without_rel_name.episode, 11);

    //
    // Dir
    //

    let beatrice_without_season = Directory::new(
        "[Beatrice-Raws] Tensura Nikki Tensei Shitara Slime Datta Ken [BDRip 1920x1080 HEVC FLAC]",
    )
    .unwrap();

    assert_eq!(beatrice_without_season.season, 1);

    let beatrice_with_season =
        Directory::new("[Beatrice-Raws] Yuru Yuri 2 [BDRip 1920x1080 HEVC FLAC]").unwrap();

    assert_eq!(beatrice_with_season.season, 2);

    let moozzi2_without_season = Directory::new(
        "[Moozzi2] Kono Subarashii Sekai ni Bakuen o! [ x265-10Bit Ver. ] - TV + SP",
    )
    .unwrap();

    assert_eq!(moozzi2_without_season.season, 1);

    let moozzi2_with_season = Directory::new(
        "[Moozzi2] Kono Subarashii Sekai ni Bakuen o! 2 [ x265-10Bit Ver. ] - TV + SP",
    )
    .unwrap();

    assert_eq!(moozzi2_with_season.season, 2);

    let subsplease_without_season =
        Directory::new("[SubsPlease] Dekiru Neko wa Kyou mo Yuuutsu (1080p)").unwrap();

    assert_eq!(subsplease_without_season.season, 1);

    let subsplease_without_season = Directory::new("[SubsPlease] Kimizero").unwrap();

    assert_eq!(subsplease_without_season.season, 1);

    let subsplease_with_season =
        Directory::new("[SubsPlease] Tsuki ga Michibiku Isekai Douchuu S2 (1080p)").unwrap();

    assert_eq!(subsplease_with_season.season, 2);
}
