#!/bin/sh

echo Cleaning Cache
rm -rf .cache

echo Downloading Paper
git clone https://github.com/PaperMC/Paper.git .cache
cd .cache/

# Versions (uncomment the version you want)
#echo Paper 1.16.2 & git checkout -b version HEAD~0
#echo Paper 1.16.1 & git checkout -b version 627f4b8561115d40d6a39587d1ad94b0104f7e14
#echo Paper 1.15.2 & git checkout -b version 81f6d51f0ea08887245b940ab685045882ab5053

BRANCH=$(git branch --show-current)
if [[ "$BRANCH" != "version" ]]; then
    echo Please choose a version by editing BuildTool-Shell.bat;
    exit 1;
fi

echo Compiling Paper
./paper jar

echo Finished.
