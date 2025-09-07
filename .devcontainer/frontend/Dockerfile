# .devcontainer/frontend/Dockerfile

# Microsoft公式のNode.js + TypeScriptプリインストール済みイメージをベースにする
# :18-bullseye の "18" はNode.jsのバージョン。プロジェクトに合わせて変更可能
FROM mcr.microsoft.com/devcontainers/typescript-node:22-bullseye

# [オプション] 他に必要なパッケージがある場合は、以下のように追記
# RUN sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
#   && sudo apt-get -y install --no-install-recommends <パッケージ名>