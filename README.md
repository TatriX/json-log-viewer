# JSON log viewer
Tiny cli app to pretty print piped logs for development purposes.

# Usage

See [Installation](#installation)
```
./app | jlv
docker logs -f server --since 1h 2>&1 | jlv
```

# Why?
So you have an app which spits JSON logs in real time. Something like
a game engine. And you want to run it pretty printing logs for
debugging purposes. This is exactly what this tool does for you.

# Installation
```
git clone https://github.com/TatriX/json-log-viewer
cd json-log-viewer
cargo build --release
cp target/release/json-log-viewer ~/.local/bin/jlv # adjust PATH accordingly
```

# Hey, I want to use it! Can you make it configurable?
Sure! Just create an issue and I'll do that.

Currently message format is hardcoded for my specific use case.
