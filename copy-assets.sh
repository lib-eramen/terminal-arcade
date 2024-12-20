#! /usr/bin/bash

assets_path=$HOME/.local/share/terminal-arcade/.assets
rm -rf $assets_path
mkdir $assets_path
cp -r ./assets/* $assets_path
echo "copied assets to ${assets_path}"