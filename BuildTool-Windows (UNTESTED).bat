@echo off
title Paper Builder
cd %~dp0

echo Cleaning Cache
rmdir /s /q ".cache"

echo Downloading Paper
git clone https://github.com/PaperMC/Paper.git .cache
cd .cache/

rem Versions (uncomment the version you want)
rem echo Paper 1.16.2 & git checkout -b version HEAD~0
rem echo Paper 1.16.1 & git checkout -b version 627f4b8561115d40d6a39587d1ad94b0104f7e14
rem echo Paper 1.15.2 & git checkout -b version 81f6d51f0ea08887245b940ab685045882ab5053

if not ${git branch --show-current} == "version" (
    echo Please choose a version by editing BuildTool-Windows.bat
    exit /b 1
)

echo Compiling Paper
reg Query "HKLM\Hardware\Description\System\CentralProcessor\0" | find /i "x86" > NUL && set OS=32BIT || set OS=64BIT
if %OS%==32BIT start "" "%SYSTEMDRIVE%\Program Files (x86)\Git\bin\sh.exe" --login -c "./paper jar"
if %OS%==64BIT start "" "%PROGRAMFILES%\Git\bin\sh.exe" --login -c "./paper jar"

echo Finished.
