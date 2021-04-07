# PaperBuilder

This is a quick and easy way of installing Paper locally as you would with Spigot / Bukkit.

## Dependencies

You will probably have all of this already, but if not..

1. Install Java 11: `https://www.oracle.com/java/technologies/javase-jdk11-downloads.html` (Recommended, login necessary)

  * or OpenJDK 11: `https://adoptopenjdk.net/`

  * *other Java providers are available, but it's recommended to avoid them unless you know what you are doing.*

2. Install git: `https://git-scm.com/downloads`

3. Install Maven: `https://maven.apache.org/download.cgi`

  * macOS: `brew install maven`

  * Linux: `sudo apt install maven`

  * Windows (Chocolatey): `choco install maven`

  * Windows: follow these instructions `https://mkyong.com/maven/how-to-install-maven-in-windows/` (tedius)

4. Install jq: `https://webinstall.dev/jq/`

## Install

Run the following commands to install Paper locally:

  * `bash installer.bash`

### Versions

You can specify a particular version by doing

`bash installer.bash --rev 1.16.5`

### Windows

If you're using Windows, you'll need to run those commands inside of Git Bash or WSL.

## Usage

Put the following in your project's dependencies
```xml
<dependency>
  <groupId>com.destroystokyo.paper</groupId>
  <artifactId>paper</artifactId>
  <version>1.16.5-R0.1-SNAPSHOT</version>
  <scope>provided</scope>
</dependency>
```
