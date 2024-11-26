# Grid Craft Launcher

> This project is under development.

Features that Grid Craft Launcher supports so far:

- [x] Microsoft OAuth2 verification
- [ ] Download
  - [x] Game download
  - [ ] Mods download
- [ ] Java management
  - [ ] Automatic detection
  - [ ] Custom directory
  - [ ] Automatically match the proper Java version to the selected Minecraft version
- [ ] Game and mods management
  - [ ] Game
  - [ ] Mods
- [ ] Game launching
  - [ ] Genius login
  - [ ] Offline login
- [ ] GUI interface
  - [ ] "Launch" page
  - [ ] "Versions" page
  - [ ] "Mods" page
  - [ ] "Settings" page
  - [ ] "About" page

## What You Should Know

To decouple UI layer and logic layer, we implement UI interface in a project while implement logic in another project. To take best experience, we use different UI framework on different platform. For Windows, we use [WinUI 3](https://learn.microsoft.com/en-us/windows/apps/winui/winui3/).

### Introduction

The **_Grid Craft Launcher_**, shorten as "GCL", is one of good alternatives to Minecraft official launcher. Benefits from Rust and C#, it is beautiful (thanks to [Fluent Design](https://fluent2.microsoft.design/)!) fast and memory-safety!

GCL is simple but powerful enough to satisfy all what you required! Game download, mods installation and management, Microsoft OAuth2 authorization...all of which are covered!

Because GCL binary build does not carry a large and clumsy Chromium, it costs less resources and brings more comforts.

### Download

The [release](https://github.com/InfyniteHeap/GCL/releases) page offers the download links of the latest version of GCL.

### License

This software is distributed under GPL-3.0 license.

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details!
