# App Launcher for Windows

![GitHub Created At](https://img.shields.io/github/created-at/bedlinger/app-launcher)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/bedlinger/app-launcher)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/bedlinger/app-launcher/total)
![GitHub License](https://img.shields.io/github/license/bedlinger/app-launcher)
![GitHub Repo stars](https://img.shields.io/github/stars/bedlinger/app-launcher)

A simple app launcher for Windows created with [Tauri](https://tauri.app/). It's a small tool that allows you to quickly launch your favorite applications.

<img src="https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/solar.png"/>

## Motivation

I created this app to learn how to use [Tauri](https://tauri.app/). It's a great tool to create desktop applications with web technologies. Also I was unhappy the windows search and wanted a simple tool to quickly launch my apps.

## Features

- Search for installed applications
- Launch applications either by clicking on them or go through the list with `Tab` and `Shift+Tab` and press enter
- Open the location of the application
- Press `Ctrl+Space` to highlight the search bar
- Press `Alt+S` to open/close the App-Launcher, but you can change this shortcut in the settings
- Settings to change the theme, the behavior when the app starts and add it to the autostart

## Infos for Developers

- `npm run tauri dev` - Start the app in development mode
- `npm run tauri build` - Build the app for production
- The frontend is located in the `src` folder, written with Vue.js and Quasar as a Component Library
- The backend is located in the `src-tauri` folder, written in Rust

## Contributions and Feedback / Questions

If you have any questions or feedback, feel free to open an issue or a pull request. I'm happy about any contributions.
