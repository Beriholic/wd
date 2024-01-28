#!/bin/bash
if ! command -v wd &>/dev/null; then
    exit
fi
DIR=$HOME/.config/wd
cargo uninstall wd
rm -r $DIR
echo "wd已卸载"
# echo "wd has been uninstalled"
