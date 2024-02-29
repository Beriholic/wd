#!/bin/bash
if ! command -v cargo &>/dev/null; then
    echo "请先安装rust"
    echo "安装可参考 Ahttps://www.rust-lang.org/zh-CN/tools/install 进行安装"
    # echo "Please install rust first"
    exit
fi

if ! command -v unzip &>/dev/null; then
    echo "请先安装unzip"
    exit
fi

if ! command -v curl &>/dev/null; then
    echo "请先安装curl"
    exit
fi

DICT_URL=https://github.com/Beriholic/wd/releases/download/v0.1.1/stardict.zip
DIR=$HOME/.config/wd
mkdir -p $DIR

donwload_database() {
    echo "正在下载离线词典数据库"
    # echo "Downloading the offline dictionary database"

    curl -LjO $DICT_URL --output $DIR/stardict.zip
    echo "正在解压离线词典数据库"
    # echo "Unzipping the offline dictionary database"
    unzip ./stardict.zip
    rm ./stardict.zip
    mv ./stardict.db $DIR/stardict.db
}

if [ -f "$DIR/stardict.db" ]; then
    echo "离线词典已存在，是否重新安装?[y/n]"
    # echo "The offline dictionary already exists, do you want to reinstall?[y/n]"
    read -r -p "" input
    case $input in
    [yY][eE][sS] | [yY])
        rm $DIR/stardict.db
        donwload_database
        ;;
    esac
else
    donwload_database
fi

echo "正在安装wd"
echo "Installing wd"
cargo install --git https://github.com/Beriholic/wd.git

echo "安装完成"
