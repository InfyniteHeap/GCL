# Grid Craft Launcher

> This project is under development.

> This repository contains all the Grid Craft Launcher components except `gridcore`, which is separately managed in an [independent repository](https://github.com/InfyniteHeap/gridcore).

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

As GCL is adapted Tauri as its GUI framework, elements in client area of GCL windows might not normally display on other operating systems because they use different webview.

* On Windows, it is [Microsoft Edge WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2), which is pre-installed on Windows 11 and Windows 10 (version 2004 and later). If you have not installed it yet or can not find it on your computer, download and install it. Please note: WebView2 only supports Windows 7 and upper versions, which means GCL can never runs on Windows Visia and other older versions.
* On MacOS, it is [WKWebView](https://developer.apple.com/documentation/webkit/wkwebview), which is pre-installed on macOS 10.10 and later. However, WKWebview does not have standalone versions. That means if your MacOS version is lower than 10.10, it is necessary to upgrade your MacOS version or buy a new Mac.
* On GNU/Linux, it is generally [WebKitGTK](https://webkitgtk.org/), but its specific version depends on the version of GNU/Linux distribution. If possible, you'd better upgrade software packages or manually download the package from its official website to avoid running into exceptions.

### Introduction

The ***Grid Craft Launcher***, shorten as "GCL", is one of good alternatives to Minecraft official launcher. Benefits from Rust, it is fast and memory-safety!

GCL is simple but powerful enough to satisfy all what you required! Game download, mods installation and management, Microsoft OAuth2 authorization...all of which are covered!

Because GCL binary build does not carry a large and clumsy Chromium, it costs less resources and brings more comforts.

GCL is also a cross-platform launcher! It can run on multiple operating systems and bring you the same experiences.

### Download

The [release](https://github.com/TheBlueAlgae/GCL/releases) page offers the download links of the latest version of GCL.

### License

This software is distributed under GPL-3.0 license.

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details!