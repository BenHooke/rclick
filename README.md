# rclick

A friendly TUI right-click menu for the terminal. Designed for people who are new to the command line — or just prefer not to memorise every command.

## Installation

### Quick install (macOS and Linux)

```sh
curl -fsSL https://raw.githubusercontent.com/BenHooke/rclick/main/install.sh | sh
```
The installer detects your OS and architecture, downloads the right binary, and places it in /usr/local/bin. It will also print the optional shell function for cd support.

### Build from source (requires Rust)

```sh
git clone https://github.com/BenHooke/rclick
cd rclick
cargo install --path .
```

## Usage

### General menu (no arguments)

```sh
rclick
```

Opens a menu with general actions for the current directory:

- **New file** — prompts for a name and runs `touch`
- **New directory** — prompts for a name and runs `mkdir -p`
- **Open…** — prompts for a path, then opens it appropriately
- **Search…** — choose between finding a file (`find`) or searching text (`grep`)
- **Copy path** — copies the current directory path to your clipboard
- **History** — shows tips for navigating recent directories

### File/directory menu

```sh
rclick somefile.txt
rclick my-folder/
```

Opens a context menu for that specific file or directory:

- **Rename** — prompts for a new name and runs `mv`
- **Copy** — prompts for a destination and runs `cp`
- **Move** — prompts for a destination and runs `mv`
- **Open** — opens in `$EDITOR` (text files), `cd` hint (directories), or `xdg-open`/`open`
- **View** — shows the file in `bat` (if installed) or `less`
- **Permissions** — friendly `chmod` picker with common presets
- **Compress** — packs into a `.tar.gz` archive
- **Delete** — confirms before running `rm` or `rm -rf`

## Navigation

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` | Select |
| `Esc` / `q` | Cancel |

## Shell function tip (for `cd` support)

Because `rclick` runs as a subprocess, it can't change your shell's working directory directly. To get seamless `cd` support when you open a directory, add this to your `.bashrc` / `.zshrc`:

```sh
rclick() {
  local tmp
  tmp=$(mktemp)
  RCLICK_CD_FILE="$tmp" command rclick "$@"
  local dir
  dir=$(cat "$tmp" 2>/dev/null)
  rm -f "$tmp"
  if [[ -n "$dir" ]]; then
    cd "$dir"
  fi
}
```

A more complete shell integration that auto-cd's is on the roadmap. As well as a more visual fuzzy finder view for the cwd.

