use std::{env::current_dir, fs, path::PathBuf};

use inquire::{
    validator::{ErrorMessage, StringValidator, Validation},
    Confirm, Text,
};
use trname::{trname_with, Directory};

fn main() {
    // /<parent>/[HorribleSubs] Kono Subarashii Sekai ni Shukufuku wo! 2 (1-12) (Batch) [1080p]
    let target_dir = std::env::args()
        .last()
        .map(|p| {
            if p.starts_with('/') || p.starts_with("./") {
                p
            } else {
                "./".to_owned() + &p
            }
        })
        .map(|p| p.parse::<PathBuf>().ok().filter(|p| p.is_dir()))
        .unwrap_or(current_dir().ok())
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

    let Directory { mut season } = Directory::new(&last_component_of_target_dir)
        .unwrap_or_else(|| Directory::from_normalized(&last_component_of_target_dir).unwrap());

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

    #[derive(Clone)]
    struct IsizeValidator {
        message: String,
    }

    impl StringValidator for IsizeValidator {
        fn validate(&self, input: &str) -> Result<Validation, inquire::CustomUserError> {
            if input.parse::<isize>().is_ok() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(ErrorMessage::Custom(
                    self.message.clone(),
                )))
            }
        }
    }

    let starts_episode_at = Text::new("starts episode at:")
        .with_initial_value("1")
        .with_validator(IsizeValidator {
            message: "invalid episode".to_owned(),
        })
        .prompt()
        .unwrap()
        .parse::<isize>()
        .unwrap_or(0);

    println!();

    let files = {
        let mut xs = Vec::new();
        for dir_entry in fs::read_dir(&target_dir).unwrap() {
            let dir_entry = dir_entry.unwrap();

            if dir_entry
                .file_name()
                .as_os_str()
                .to_string_lossy()
                .starts_with('.')
            {
                continue;
            }

            if dir_entry.metadata().unwrap().is_file() {
                let file_name = dir_entry.file_name().into_string().unwrap();

                if let Some((_file, after)) =
                    trname_with(&title, season, &file_name, starts_episode_at)
                {
                    xs.push((after, dir_entry));
                }
            }
        }

        xs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        xs
    };

    for (modified_file_name, file) in files.iter() {
        let modified_target = target_dir.join(modified_file_name);

        if !modified_target.exists() {
            println!(
                "{:?} -> {:?}",
                file.file_name().as_os_str().to_string_lossy(),
                modified_file_name
            );
        }

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
                fs::rename(file.path(), &modified_target).unwrap();
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
