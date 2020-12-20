const fetch = require("node-fetch");
const { sprintf } = require("sprintf-js");
const chalk = require("chalk");
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const version_regex = /(?:-v|--version)=([0-9pre.-]+)/giu;
const exec_options = { encoding: "utf8", stdio: "inherit", windowsHide: true };
const api_projects = "https://papermc.io/api/v2/projects/paper/";
const api_versions = "https://papermc.io/api/v2/projects/paper/versions/%s";
const api_builds = "https://papermc.io/api/v2/projects/paper/versions/%s/builds/%s";
const paper_git = "https://github.com/PaperMC/Paper.git";
const paper_dir = "paper";

(async function() {
    console.log(`Paper Downloader`);

    // Determine the version of Paper to download
    let version = null;
    for (const [ignored, value] of process.argv.entries()) {
        const result = [...value.matchAll(version_regex)];
        if (result.length > 0) {
            version = result[0][1];
            console.log(`Version: ${version} (arguments)`);
            break;
        }
    }
    if (version === null) {
        const versions = (await fetch(api_projects)
                .then(res => res.json())
                .catch(() => {}) ?? {})
                ["versions"] ?? [];
        if (versions.length > 0) {
            version = versions[versions.length - 1];
            console.log(`Version: ${version} (default)`);
        }
    }
    if (version === null) {
        console.warn(chalk.yellow("Version: could not be determined!"));
        process.exit(1);
    }

    // Figure out what build to use for that version
    const builds = (await fetch(sprintf(api_versions, version))
            .then(res => res.json())
            .catch(() => {}) ?? {})
            ["builds"] ?? [];
    if (builds.length < 1) {
        console.warn(chalk.yellow("Could not find any builds for that version. Are you sure it's supported?"));
        process.exit(1);
    }
    const build = builds[builds.length - 1];
    console.log(`Build: ${build}`);

    // Figure out what commit to clone
    const commits = (await fetch(sprintf(api_builds, version, build))
            .then(res => res.json())
            .catch(() => {}) ?? {})
            ["changes"] ?? [];
    if (commits.length < 1) {
        console.warn(chalk.yellow("Could not find any commits for that version. Are you sure it's supported?"));
        process.exit(1);
    }
    const commit = commits[commits.length - 1]["commit"];
    console.log(`Commit: ${commit}`);

    // Remove any previous install files
    const paperLocation = path.resolve(__dirname, paper_dir);
    if (fs.existsSync(paperLocation)) {
        try {
            fs.rmdirSync(paperLocation, { recursive: true });
        }
        catch (ignored) {
            console.warn(chalk.yellow(`Could not remove clutter. Please remove the ${paper_dir} directory!`));
            process.exit(1);
        }
    }

    // Set up Paper's git stuff
    console.log("Setting up download");
    try {
        execSync("git init " + paper_dir,
                { cwd: __dirname, ...exec_options });
        execSync("git config core.autocrlf false",
                { cwd: paperLocation, ...exec_options });
        execSync("git remote add origin " + paper_git,
                { cwd: paperLocation, ...exec_options });
        execSync("git fetch --all",
                { cwd: paperLocation, ...exec_options });
        execSync("git reset --hard " + commit,
                { cwd: paperLocation, ...exec_options });
    }
    catch (ignored) {
        console.warn(chalk.yellow(`Could not initalise Paper's git folder!`));
        process.exit(1);
    }

    // Install Paper
    console.log("Installing Paper");
    try {
        execSync("./paper jar", { cwd: paperLocation, ...exec_options });
    }
    catch (ignored) {
        console.warn(chalk.yellow("Could not build Paper!"));
        process.exit(1);
    }

    console.warn(chalk.blue("Done :)"));
}());
