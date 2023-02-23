#!/bin/bash

cargo build
mv target/debug/discordbot ~/bin/discordinit && chmod +x ~/bin/discordinit && echo "Moved to bin!"