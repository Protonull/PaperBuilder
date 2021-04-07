#!/bin/bash
cd "$(dirname "$0")" || exit

yellow=$'\e[0;33m'
blue=$'\e[1;34m'

throw() {
    echo "$*" >&2
    exit 1
}

version=0
userDefined=0

# Attempt to get the version from the arguments
while [ $# -gt 0 ]; do
    if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        if [[ $param == "rev" ]]; then
            declare version="$2"
            declare userDefined=1
        fi
    fi
    shift
done
# Otherwise default to the newest version
if [[ $version == 0 ]]; then
    declare version=$(curl -Ss https://papermc.io/api/v2/projects/paper/ | jq -r '.versions[-1] // 0')
fi
# If version is still empty, something went horribly wrong
if [[ $version == 0 ]]; then
    throw "${yellow}Version: could not be determined!"
fi
# Otherwise output found version
if [[ $userDefined == 0 ]]; then
    echo "Version: ${version} (default)"
else
    echo "Version: ${version} (arguments)"
fi

# Attempt to get the build number from the version
build=$(curl -Ss https://papermc.io/api/v2/projects/paper/versions/${version} | jq -r '.builds[-1] // 0')
# If the build number is empty, something went wrong
if [[ $build == 0 ]]; then
    throw "${yellow}Build: could not be determined!"
fi
echo "Build: ${build}"

# Attempt to get the build number from the version
commit=$(curl -Ss https://papermc.io/api/v2/projects/paper/versions/${version}/builds/${build} | jq -r '.changes[-1].commit // 0')
# If the build number is empty, something went wrong
if [[ $commit == 0 ]]; then
    throw "${yellow}Commit: could not be determined!"
fi
echo "Commit: ${commit}"

paperDir="paper"
paperGit="https://github.com/PaperMC/Paper.git"

rm -rf ${paperDir}

echo "Setting up download"
git init ${paperDir} -q
(cd $paperDir;
    git config core.autocrlf false;
    git remote add origin $paperGit;
    git fetch -q origin master;
    git reset -q --hard $commit;
    echo "Installing Paper";
    ./paper build)

echo "${blue}Done :)"