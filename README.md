A complete rewrite of @XNBlank 's sudobot. This bot aims to be feature-complete with the previous version, while having complete reliability in terms of moderation.

Installation:
```bash
# Clone the repo.
git clone https://github.com/too-r/sudobot-rs.git
# Wherever you put it
cd $HOME/cloned_repo
# Build with cargo (Ignore warnings)
cargo build
```

Eventually this bot will have the ability to run as some sort of system service and I will update this as such.

Before running, copy the `config.example.toml` to `~/.config/sudobot/config.tonl` and populate. To retrieve the id of the channel you want sudobot to log moderation events to:
- Turn on developer mode in Discord (Settings -> Appearance -> Advanced -> Developer Mode).
- Right click on the desired channel and click on `Copy ID` at the bottom of the context menu that appears.

You can then run it:
```bash
cd $HOME/cloned_repo
cargo run
```

