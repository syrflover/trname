use std::{env::current_dir, fs, path::PathBuf};

use inquire::{
    validator::{ErrorMessage, StringValidator, Validation},
    Confirm, Text,
};
use regex::Regex;

fn main() {
    // /<parent>/[HorribleSubs] Kono Subarashii Sekai ni Shukufuku wo! 2 (1-12) (Batch) [1080p]
    let target_dir = std::env::args()
        .find_map(|p| p.parse::<PathBuf>().ok().filter(|p| p.is_dir()))
        .or(current_dir().ok())
        .expect("invalid path");

    println!("{}", target_dir.as_os_str().to_string_lossy());

    let title = target_dir
        .parent()
        .unwrap()
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .into_owned();

    println!();
    println!("{}", title);
    println!();

    let last_component_of_target_dir = target_dir
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy();

    let DirMetadata { mut season } = DirMetadata::new(&last_component_of_target_dir)
        .unwrap_or_else(|| DirMetadata::from_normalized(&last_component_of_target_dir).unwrap());

    #[derive(Clone)]
    struct UsizeValidator {
        message: String,
    }

    impl StringValidator for UsizeValidator {
        fn validate(&self, input: &str) -> Result<Validation, inquire::CustomUserError> {
            if input.parse::<usize>().is_ok() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(ErrorMessage::Custom(
                    self.message.clone(),
                )))
            }
        }
    }

    {
        let text = Text::new("season:")
            .with_initial_value(&season.to_string())
            .with_validator(UsizeValidator {
                message: "invalid season".to_owned(),
            })
            .prompt()
            .unwrap();

        season = text.parse().unwrap();
    }

    println!();

    let files = {
        let mut xs = Vec::new();
        for x in fs::read_dir(&target_dir).unwrap() {
            let x = x.unwrap();

            if x.file_name().as_os_str().to_string_lossy().starts_with('.') {
                continue;
            }

            if x.metadata().unwrap().is_file() {
                let file_name = x.file_name().into_string().unwrap();

                if let Some(FileMetadata { episode, ext }) = FileMetadata::new(&file_name) {
                    let after = format!(
                        "{} S{}E{}.{}",
                        title,
                        into_least_two_chars(season),
                        into_least_two_chars(episode),
                        ext
                    );

                    xs.push((after, x));
                }
            }
        }

        xs.sort_by(|a, b| a.0.cmp(&b.0));

        xs
    };

    for (modified_file_name, file) in files.iter() {
        println!(
            "{:?} -> {:?}",
            file.file_name().as_os_str().to_string_lossy(),
            modified_file_name
        );

        // 사용법
        // 1. 제목으로 이루어진 폴더를 만든다 -> Kono Subarashii Sekai ni Shukufuku wo!
        // 2. 방금 만든 폴더 안에 애니 폴더를 넣는다 -> Kono Subarashii Sekai ni Shukufuku wo!/[HorribleSubs] Kono Subarashii Sekai ni Shukufuku wo! [1080p]
        // 3. renamer ./Kono Subarashii Sekai ni Shukufuku wo!/[HorribleSubs] Kono Subarashii Sekai ni Shukufuku wo! [1080p]
    }

    let modified_target_dir = target_dir
        .parent()
        .unwrap()
        .join(format!("Season {}", into_least_two_chars(season)));

    if !modified_target_dir.exists() {
        println!(
            "{:?} -> {:?}",
            target_dir
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy(),
            modified_target_dir
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
        );
    }

    println!();
    println!("title: {}", title);
    println!("season: {}", season);
    println!();

    let confirm = Confirm::new("confirm [y/n]:").prompt().unwrap();

    if confirm {
        for (modified_file_name, file) in files {
            let modified_target = target_dir.join(modified_file_name);

            if !modified_target.exists() {
                fs::rename(&file.path(), &modified_target).unwrap();
            }
        }

        if !modified_target_dir.exists() {
            fs::rename(&target_dir, &modified_target_dir).unwrap();
        }
    }
}

pub fn into_least_two_chars(x: usize) -> String {
    let x = x.to_string();
    if x.chars().count() == 1 {
        "0".to_owned() + &x
    } else {
        x
    }
}

struct DirMetadata {
    season: usize,
}

impl DirMetadata {
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

struct FileMetadata {
    episode: usize,
    ext: String,
}

impl FileMetadata {
    pub fn new(s: &str) -> Option<Self> {
        let regex = Regex::new(
            r"(?i)^\[.+\]\s(.+)\s(?<episode>[0-9]+)(\s[a-z]+)?(\s\(.+\))?(\s\[.+\])?\.(?<ext>\w+)$",
        )
        .unwrap();

        let regex_without_rel_name = Regex::new(r"(?<episode>[0-9]+).*\.(?<ext>\w+)$").unwrap();

        let captured = regex.captures(s).or(regex_without_rel_name.captures(s))?;

        fn parse_episode(s: &str) -> Option<usize> {
            if s.starts_with('0') {
                return parse_episode(&s[1..]);
            } else {
                &s
            }
            .parse()
            .ok()
        }

        let episode = parse_episode(&captured["episode"])?;

        let ext = captured["ext"].to_owned();

        Some(Self { episode, ext })
    }
}

#[test]
fn tests_regex() {
    //
    // File
    //

    let beatrice = FileMetadata::new(
        "[Beatrice-Raws] Kono Subarashii Sekai ni Shukufuku wo! 04 (BDRip 1920x1080 x264 FLAC).mkv",
    )
    .unwrap();

    assert_eq!(beatrice.episode, 4);

    let subsplease = FileMetadata::new(
        "[SubsPlease] Kono Subarashii Sekai ni Bakuen wo! - 01 (1080p) [10709A2B].mkv",
    )
    .unwrap();

    assert_eq!(subsplease.episode, 1);

    let moozzi2 = FileMetadata::new(
        "[Moozzi2] Fullmetal Alchemist Brotherhood - 64 END (BD 1920x1080 x.264 Flac).mkv",
    )
    .unwrap();

    assert_eq!(moozzi2.episode, 64);

    let without_rel_name =
        FileMetadata::new("Kono Subarashii Sekai ni Bakuen wo! - 01LoremIpsum.smi").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = FileMetadata::new("프리렌 1.ass").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = FileMetadata::new("프리렌 1 (F).ass").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = FileMetadata::new("nogame01.ass").unwrap();

    assert_eq!(without_rel_name.episode, 1);

    let without_rel_name = FileMetadata::new("nogame12.ass").unwrap();

    assert_eq!(without_rel_name.episode, 12);

    //
    // Dir
    //

    let beatrice_without_season = DirMetadata::new(
        "[Beatrice-Raws] Tensura Nikki Tensei Shitara Slime Datta Ken [BDRip 1920x1080 HEVC FLAC]",
    )
    .unwrap();

    assert_eq!(beatrice_without_season.season, 1);

    let beatrice_with_season =
        DirMetadata::new("[Beatrice-Raws] Yuru Yuri 2 [BDRip 1920x1080 HEVC FLAC]").unwrap();

    assert_eq!(beatrice_with_season.season, 2);

    let moozzi2_without_season = DirMetadata::new(
        "[Moozzi2] Kono Subarashii Sekai ni Bakuen o! [ x265-10Bit Ver. ] - TV + SP",
    )
    .unwrap();

    assert_eq!(moozzi2_without_season.season, 1);

    let moozzi2_with_season = DirMetadata::new(
        "[Moozzi2] Kono Subarashii Sekai ni Bakuen o! 2 [ x265-10Bit Ver. ] - TV + SP",
    )
    .unwrap();

    assert_eq!(moozzi2_with_season.season, 2);

    let subsplease_without_season =
        DirMetadata::new("[SubsPlease] Dekiru Neko wa Kyou mo Yuuutsu (1080p)").unwrap();

    assert_eq!(subsplease_without_season.season, 1);

    let subsplease_without_season = DirMetadata::new("[SubsPlease] Kimizero").unwrap();

    assert_eq!(subsplease_without_season.season, 1);

    let subsplease_with_season =
        DirMetadata::new("[SubsPlease] Tsuki ga Michibiku Isekai Douchuu S2 (1080p)").unwrap();

    assert_eq!(subsplease_with_season.season, 2);
}
