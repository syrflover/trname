use std::path::Path;

mod directory;
mod file;

pub use directory::Directory;
pub use file::File;

/// ./media/love live/Season 01
///
/// return: love live S01E01.mkv
pub fn trname(path: impl AsRef<Path>, file_name: &str, starts_episode_at: isize) -> Option<String> {
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

    println!("{title} {season}");

    let Directory { season } = Directory::from_normalized(&season)?;

    let File { mut episode, ext } = File::new(&title, file_name)?;

    episode = episode.checked_add_signed(if starts_episode_at.is_negative() {
        starts_episode_at
    } else if starts_episode_at.is_positive() {
        starts_episode_at - 1
    } else {
        0
    })?;

    let after = format!(
        "{} S{}E{}.{}",
        title,
        into_least_two_chars(season),
        into_least_two_chars(episode),
        ext
    );

    Some(after)
}

fn into_least_two_chars(x: usize) -> String {
    let x = x.to_string();
    if x.chars().count() == 1 {
        "0".to_owned() + &x
    } else {
        x
    }
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
}
