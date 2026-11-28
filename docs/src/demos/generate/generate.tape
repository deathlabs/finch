# This tape requires the suppression generator. The Rust rewrite does not provide it yet.
# Run from the repository root with Finch, Grype, VHS, ttyd, and ffmpeg installed.
# Grype needs a vulnerability database containing GHSA-jfh8-c2jp-5v3q.
Output docs/src/content/user-guide/assets/generate.gif

Set Shell "bash"
Set Theme "Dracula"
Set FontSize 20
Set Width 1400
Set Height 700
Set TypingSpeed 12ms

Hide
Type "export GRYPE_DB_AUTO_UPDATE=false GRYPE_DB_VALIDATE_BY_HASH_ON_START=false; sbom=sbom:docs/src/demos/generate/log4j-sbom.json; demo_dir=$(mktemp -d); clear"
Enter
Sleep 1s
Show
Type "grype $sbom -q | grep --color=always -E 'NAME|GHSA-jfh8-c2jp-5v3q'"
Enter
Sleep 4s
Type "finch generate --mappings docs/src/demos/generate/mappings.yaml --tool grype --output-dir $demo_dir"
Enter
Sleep 3s
Type "grype $sbom -q --config $demo_dir/grype-ignore.yaml --show-suppressed | grep --color=always -E 'NAME|GHSA-jfh8-c2jp-5v3q|suppressed'"
Enter
Sleep 6s
