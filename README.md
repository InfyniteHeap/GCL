# Grid Craft Launcher

>This project is under development.

>This repository includes all of GCL source code except "gridcore", which is seperately managed in an [independent repository](https://github.com/TheBlueAlgae/gridcore).

**Note: This program is provided "as-is" so there are no guarantees that this source code follows standards and compliance.**

The process of developing GCL:

- [x] Microsoft OAuth2 verification (**Roughly completed!**)
- [ ] Download
    - [ ] Game download
    - [ ] Mods download
- [ ] Java management
    - [ ] Automatically detect
    - [ ] Custom directory
    - [ ] Automatically match the proper Java version to selected Minecraft version
- [ ] Game and mods management
    - [ ] Game
    - [ ] Mods
- [ ] Game launch
    - [ ] Genius login
    - [ ] Offline login
- [ ] Windows interface
    - [ ] "Launch" page
    - [ ] "Versions" page
    - [ ] "Mods" page
    - [ ] "Settings" page
    - [ ] "About" page

## What You Should Know

Since GCL used Tauri as its GUI framework, elements in client area of GCL windows might not normally display on other operating systems because they use different webview.

* On Windows, it is [Microsoft Edge WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2), which is pre-installed on Windows 11 and Windows 10 (version 2004 and later). If you have not installed it yet or can not find it on your computer, download and install it. Please note: WebView2 only supports Windows 7 and upper versions, which means GCL can never runs on Windows XP and other older Windows.
* On MacOS, it is [WKWebView](https://developer.apple.com/documentation/webkit/wkwebview), which is pre-installed on macOS 10.10 and later. However, WKWebview does not have standalone versions. That means if your MacOS version is lower than 10.10, it is necessary to upgrade your MacOS version or buy a new Mac.
* On GNU/Linux, it is generally [WebKitGTK](https://webkitgtk.org/), but its specific version depends on the version of GNU/Linux distribution. If possible, you'd better upgrade software packages or manually download the package from its official website to avoid running into exceptions.

### Introduction

The ***Grid Craft Launcher***, also called "GCL", is one of good alternatives of Minecraft official launcher. Thanks to Rust, it is fast and memory-safety!

GCL is simple but powerful enough to match all what you required! Game download, mods installation and management, Microsoft OAuth2 authriozation...all of which are covered!

Because GCL binary build does not carry a large and clumsy Chromium, it costs less resources and brings more comforts.

GCL is also a cross-platform launcher! It can run on mutiple operating systems and bring you the same experiences.

### Download

The [release](https://github.com/TheBlueAlgae/GCL/releases) page offers the download links of the latest version of GCL.

### License

This software is distributed under GPL-3.0 license.

### Contribution

Greeting for your contributions! I always believe that the root of open-source softwares is **community**! What to make GCL better? Just commit your pull requests or issues!