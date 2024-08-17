use std::path::Path;

mod directory;
mod file;

pub use directory::Directory;
pub use file::File;

pub fn trname_with(
    title: &str,
    season: usize,
    file_name: &str,
    starts_episode_at: isize,
) -> Option<(File, String)> {
    let mut file = File::new(&title, file_name)?;

    if starts_episode_at.is_negative() {
        file.episode += starts_episode_at as f32;
    } else if starts_episode_at.is_positive() {
        file.episode += (starts_episode_at - 1) as f32;
    }

    // episode = episode.checked_add_signed()?;

    let after = format!(
        "{} S{}E{}.{}",
        title,
        format_with_leading_zero(season as f32),
        format_with_leading_zero(file.episode),
        file.ext
    );

    Some((file, after))
}

pub fn trname_raw(
    path: impl AsRef<Path>,
    file_name: &str,
    starts_episode_at: isize,
) -> Option<(Directory, File, String)> {
    let path = path.as_ref();
    let title = path
        .components()
        .rev()
        .nth(1)?
        .as_os_str()
        .to_os_string()
        .into_string()
        .ok()?;
    let season = path
        .components()
        .last()?
        .as_os_str()
        .to_os_string()
        .into_string()
        .ok()?;

    // println!("{title} {season}");

    let directory = Directory::from_normalized(&season)?;

    let (file, after) = trname_with(&title, directory.season, file_name, starts_episode_at)?;

    Some((directory, file, after))
}

/// ./media/love live/Season 01
///
/// return: love live S01E01.mkv
pub fn trname(path: impl AsRef<Path>, file_name: &str, starts_episode_at: isize) -> Option<String> {
    trname_raw(path, file_name, starts_episode_at).map(|x| x.2)
}

fn format_with_leading_zero(x: f32) -> String {
    let is_less_than_10 = x < 10.0;
    // let is_fraction_zero = x.fract() == 0.0;

    let mut x = x.to_string();

    // println!("{x}");

    if is_less_than_10 {
        x = "0".to_owned() + &x;
    }

    // if is_fraction_zero {
    //     x = x[..x.len() - 2].to_owned();
    // }

    x
}

#[test]
fn test_trname() {
    use std::path::PathBuf;

    let actual = trname(
        PathBuf::from("./media/Shows (current)/Giji Harem/Season 01"),
        "[SubsPlease] Giji Harem - 02 (1080p) [0506461C].mkv",
        1,
    );

    assert_eq!(actual.unwrap(), "Giji Harem S01E02.mkv");

    let actual = trname(
        PathBuf::from("./media/Shows (current)/Tensei Shitara Slime Datta Ken/Season 03"),
        "[SubsPlease] Tensei Shitara Slime Datta Ken - 62 (1080p) [0214B01E].mkv",
        -48,
    );

    assert_eq!(actual.unwrap(), "Tensei Shitara Slime Datta Ken S03E14.mkv");

    let actual = trname(
        PathBuf::from("./media/Shows (current)/Tensei Shitara Slime Datta Ken/Season 03"),
        "[SubsPlease] Tensei Shitara Slime Datta Ken - 65.5 (1080p) [0214B01E].mkv",
        -48,
    );

    assert_eq!(
        actual.unwrap(),
        "Tensei Shitara Slime Datta Ken S03E17.5.mkv"
    );
}
