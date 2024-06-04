# Up

A very simple rust program that updates all your software when you need it.

**TL;DR:** `up` launches a bunch of commands you can't be bothered to remember
concurrently so it goes faster!

![Video of the software in action](./.github/videos/output.gif)

Instead of typing five different commands to update your distro's package
manager, rustup, pip and node, you just run `up` and it launches everything at
once.

Up uses [rayon](https://crates.io/crates/rayon) for multi-threading.

## Features / Roadmap

- [x] Read list of commands from standard location (ex: XDG user directory on
  linux)
- [x] Select list of commands from the cli
- [ ] Create a config file if none exists
- [ ] Create multiple command groups and select them from the cli
- [ ] Runs reliably in the background with a simple way to tell that something
  failed
- [ ] Write a cron job
- [ ] Colors and loading icon

## Installation

The software is currently in alpha, so you'll need to build it yourself.
Thankfully, there are no special dependencies, you just need the
[Rust](https://rustup.rs/) toolchain.

```bash
git clone https://github.com/paul-ohl/Up.git
cd Up/
cargo build --release
```

Then you can copy it to a folder in your `$PATH` to run it from anywhere.

```bash
sudo cp ./target/release/up /usr/bin/

# Or for user-level install
cp ./target/release/up ~/.local/bin/
# You may need to add this directory to your path
export PATH="$PATH:$HOME/.local/bin" # <-- in your ~/.bashrc or ~/.zshrc
```

## Configuration

All you need to do is create a file in your XDG user directory.

The following tutorial is for linux, but works the exact same on MacOS, and you
only need to adjust the directory for Windows.

```bash
mkdir ~/.config/up/
touch ~/.config/up/commands.toml
```

Then add commands to the `commands.toml` file. For example:

```toml
# In your commands.toml
system = "sudo dnf update -y"
flatpak = "flatpak update -y"
rustup = "rustup update"
snap = "sudo snap refresh"
lazyvim = "nvim --headless '+Lazy! sync' +qa"
npm = "sudo npm update -g"
pip = "pip install --upgrade pip"
```
Make sure your commands don't need any interaction to run (notice the `-y`).

Then once your commands are ready, just run `up`!
