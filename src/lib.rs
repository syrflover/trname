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
    let mut file = File::new(title, file_name)?;

    if starts_episode_at.is_negative() {
        let episode = file.episode + starts_episode_at as f32;

        if episode >= 1.0 {
            file.episode = episode;
        }
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
        .next_back()?
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

    let actual = trname(
        PathBuf::from("./media/Shows (current)/Tensei Shitara Slime Datta Ken/Season 03"),
        "[SubsPlease] Tensei Shitara Slime Datta Ken - 67 (1080p) [898A0134].mkv",
        -48,
    );

    assert_eq!(actual.unwrap(), "Tensei Shitara Slime Datta Ken S03E19.mkv");
}

#[test]
fn test_trname_long_and_half_episodes() {
    use std::path::PathBuf;

    let one_piece = PathBuf::from("./media/Shows (current)/One Piece/Season 01");
    let show = PathBuf::from("./media/Shows (current)/Show/Season 01");

    for (path, file_name, expected) in [
        (
            &one_piece,
            "[SubsPlease] One Piece - 1000 (1080p) [ABCD1234].mkv",
            "One Piece S01E1000.mkv",
        ),
        (
            &one_piece,
            "[SubsPlease] One Piece - 1001 (1080p) [ABCD1234].mkv",
            "One Piece S01E1001.mkv",
        ),
        (&show, "[Moozzi2] Show - 105 (BD).mkv", "Show S01E105.mkv"),
        (&show, "[Group] Show - 05.5 (1080p).mkv", "Show S01E05.5.mkv"),
        (&show, "[Group] Show - 105 (1080p).mkv", "Show S01E105.mkv"),
        (
            &show,
            "[Ioroid] Show - 105 [AMZN WEB-DL 1080p AVC E-AC3].mkv",
            "Show S01E105.mkv",
        ),
        (&show, "Show S01E105.mkv", "Show S01E105.mkv"),
        (&one_piece, "One Piece S01E1000.mkv", "One Piece S01E1000.mkv"),
    ] {
        let actual = trname(path, file_name, 1);

        assert_eq!(actual.as_deref(), Some(expected), "{file_name}");
    }

    let formatted = File::new("Show", "Show S01E105.mkv").unwrap();

    assert_eq!(formatted.episode, 105.0);
    assert!(formatted.already_formatted);
}
