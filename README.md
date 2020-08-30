# PaperBuilder

This project is intended to be the Paper version of BuildTools where you select a version and it'll compile and install the resulting Paper jar. This has the added benefit of being able to replace, for example:
```xml
<dependency>
  <groupId>org.spigotmc</groupId>
  <artifactId>spigot</artifactId>
  <version>1.16.1-R0.1-SNAPSHOT</version>
  <scope>provided</scope>
</dependency>
```
...with:
```xml
<dependency>
  <groupId>com.destroystokyo.paper</groupId>
  <artifactId>paper</artifactId>
  <version>1.16.1-R0.1-SNAPSHOT</version>
  <scope>provided</scope>
</dependency>
```
which then grants you access to the deobfuscation efforts and any custom features added to Paper's server code.
