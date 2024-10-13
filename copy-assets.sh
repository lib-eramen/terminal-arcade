#! /usr/bin/bash

assets_path=$HOME/.local/share/terminal-arcade/.assets
if [ ! -d $assets_path ]; then
    mkdir $assets_path
else
    rm -rf $assets_path
fi
cp -r ./assets/* $assets_path
