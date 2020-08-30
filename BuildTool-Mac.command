#!/bin/sh
set echo off
echo -n -e "\033]0;Paper Builder\007"
cd "$(dirname "$0")"
./BuildTool-Shell.sh
