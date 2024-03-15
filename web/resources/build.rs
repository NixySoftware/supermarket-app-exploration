use std::{env, fs, io, path::PathBuf};

const ICONS: &[&str] = &["shopping-cart"];
const ICON_RENAMES: &[(&str, &str)] = &[];

const PROVIDER_ICONS: &[&str] = &["apple", "azure", "google"];
const PROVIDER_ICON_RENAMES: &[(&str, &str)] = &[("azure", "microsoft")];

fn main() -> Result<(), io::Error> {
    let current_path = env::current_dir()?;

    let from_path = current_path.join("../../node_modules/lucide-static/icons");
    let to_path = current_path.join("../public/images/icons");
    copy_icons(from_path, to_path, ICONS, ICON_RENAMES)?;

    let from_path = current_path.join("../../node_modules/next-auth/docs/static/img/providers");
    let to_path = current_path.join("../public/images/icons/providers");
    copy_icons(from_path, to_path, PROVIDER_ICONS, PROVIDER_ICON_RENAMES)?;

    Ok(())
}

fn copy_icons(
    from_path: PathBuf,
    to_path: PathBuf,
    icons: &[&str],
    renames: &[(&str, &str)],
) -> Result<(), io::Error> {
    // Copy icons
    for icon in icons {
        let to_icon = match renames.iter().find(|(from, _)| from == icon) {
            Some((_, to)) => to,
            None => icon,
        };

        fs::copy(
            from_path.join(format!("{}.svg", icon)),
            to_path.join(format!("{}.svg", to_icon)),
        )?;
    }

    // Remove unused icons
    for entry in fs::read_dir(&to_path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let icon = &path.file_stem().unwrap().to_str().unwrap();

        if renames.iter().any(|(from, _)| from == icon) {
            fs::remove_file(path)?;
            continue;
        }

        let from_icon = match renames.iter().find(|(_, to)| to == icon) {
            Some((from, _)) => from,
            None => icon,
        };

        if !icons.contains(from_icon) {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
