# Rustプリインストール済みのイメージをベースにする
FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# (例) 将来的にWebAssemblyを扱うためにwasm-packをインストール
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# (例) 他のシステムライブラリが必要な場合
# RUN sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
#   && sudo apt-get -y install --no-install-recommends <パッケージ名>