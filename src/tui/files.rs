use std::path::Path;

use walkdir::WalkDir;

pub(super) fn completions(prefix: &str) -> Result<Vec<String>, walkdir::Error> {
    let root: &Path = Path::new(".");
    let mut matches: Vec<String> = Vec::new();
    for entry in WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| entry.file_name() != ".git" && entry.file_name() != "target")
    {
        let entry: walkdir::DirEntry = entry?;
        if !entry.file_type().is_file()
            || !entry
                .path()
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            continue;
        }
        let path: &Path = entry.path();
        let path: &Path = if prefix.starts_with("./") {
            path
        } else {
            path.strip_prefix(root).unwrap_or(path)
        };
        if let Some(path) = path.to_str()
            && path.starts_with(prefix)
        {
            matches.push(path.to_owned());
        }
    }
    return Ok(matches);
}
