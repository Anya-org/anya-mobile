#!/bin/bash
set -e

sudo apt-get update
sudo apt-get install -y \
    libglib2.0-dev \
    libgtk-3-dev \
    protobuf-compiler \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libxdo-dev
