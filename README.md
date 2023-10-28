[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![rust-tests-shield]][rust-tests-url]
[![docs-build-shield]][docs-build-url]  
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

Quick Navigation:

- [Description](#description)
- [How to run the project locally ?](#how-to-run-the-project-locally-)
- [How it came to be ?](#how-it-came-to-be-)
- [Features different from other editors](#features-different-from-other-editors)
- [Technologies used](#technologies-used)
- [License](#license)
- [Author](#author)

## Description

This project is an Intel 8086 emulator / vm that can run most of the 8086 instruction set and provides an interactive interpreter to run the program line by line. The repository contains the core library, which includes the preprocessor, data parser, and interpreter.

Features:

- Emulates most of the 8086 instruction set
- Provides an interactive interpreter to run the program line by line
- Includes a preprocessor to expand macros and resolve labels
- Includes a data parser to convert the preprocessed assembly language code into a data structure that can be understood by the interpreter
- Includes an interpreter to execute the assembly language code one line at a time

Brief Overview:
    Using `rust` as the backend for driving the emulator and `react` as the frontend for the web app, the project is divided into two parts:
    
  - core: The core library that contains the preprocessor, data parser, and interpreter, this is the `src-tauri` folder.

  - ui: The web app that provides an interactive interface to run the emulator, this is the `src` folder.

## How to run the project locally ? 

1. Clone the repository
```bash
git clone https://github.com/Sarath191181208/emu_8086
```

2. Install the dependencies
```bash
cd emu_8086
npm i
```

3. Run the project
```bash
npm run tauri dev
```


## How it came to be ?
In collage we have a course called Computer Organization and Architecture. It was a course where you know how the computers came to be and how they work in different ways. How the processor works, how the memory, disk, etc work. We also have a project where to have to build a simple assembly program which is given as  a problem statement by our professor. Intrigued by how the 8086 and how it works, I tried to make it from scratch. Altough I didn't have enough hardware, I sought to make it in software. I started with the preprocessor, then the data parser, and finally the interrupts.

## Features different from other editors 
- The emulator is written in rust, which is a fast and safe language.
- High focus on the editing experience.
- Easy and redable syntax highlighting. 
- Easy to use and interactive interface.
- Provides an interactive interpreter to run the program line by line.
- Easy and intuitive way to debug the program.

## Technologies used
[![Rust][Rust-shield]][Rust-url] 
[![Tauri][Tauri-shield]][Tauri-url]
[![React][React-shield]][React-url]
[![Tailwind CSS][Tailwind-shield]][Tailwind-url]
[![Typescript][Typescript-shield]][Typescript-url]

## License
This project has been licensed under MIT License. Please see the [LICENSE](LICENSE) file for more details.

## Author
[Vangipuram Srinivasa Sarath Chandra](https://github.com/Sarath191181208)

<!-- MARKDOWN LINKS && Images -->
[contributors-shield]: https://img.shields.io/github/contributors/Sarath191181208/emu_8086.svg?style=for-the-badge
[contributors-url]: https://github.com/Sarath191181208/emu_8086/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Sarath191181208/emu_8086.svg?style=for-the-badge
[forks-url]: https://github.com/Sarath191181208/emu_8086/network/members
[stars-shield]: https://img.shields.io/github/stars/Sarath191181208/emu_8086.svg?style=for-the-badge
[stars-url]: https://github.com/Sarath191181208/emu_8086/stargazers
[issues-shield]: https://img.shields.io/github/issues/Sarath191181208/emu_8086.svg?style=for-the-badge&color=%2300735a
[issues-url]: https://github.com/Sarath191181208/emu_8086/issues
[license-shield]: https://img.shields.io/github/license/Sarath191181208/emu_8086.svg?style=for-the-badge
[license-url]: https://github.com/Sarath191181208/emu_8086/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/sarath191181208/
[rust-tests-shield]: https://img.shields.io/github/actions/workflow/status/Sarath191181208/emu_8086/rust.yml?style=for-the-badge&label=Rust%20Tests&color=%2300735a
[rust-tests-url]: https://github.com/Sarath191181208/emu_8086/actions/workflows/rust.yml
[docs-build-shield]: https://img.shields.io/github/actions/workflow/status/Sarath191181208/emu_8086/docs.yml?style=for-the-badge&label=Docs%20Build&color=%2300735a
[docs-build-url]: https://github.com/Sarath191181208/emu_8086/actions/workflows/docs.yml

[Rust-shield]: https://img.shields.io/badge/-Rust-black.svg?style=for-the-badge&logo=rust&colorB=555
[Rust-url]: https://www.rust-lang.org/
[React-shield]: https://img.shields.io/badge/-React-black.svg?style=for-the-badge&logo=react&colorB=555
[React-url]: https://reactjs.org/
[Tailwind-shield]: https://img.shields.io/badge/-Tailwind-black.svg?style=for-the-badge&logo=tailwindcss&colorB=555
[Tailwind-url]: https://tailwindcss.com/
[Typescript-shield]: https://img.shields.io/badge/-Typescript-black.svg?style=for-the-badge&logo=typescript&colorB=555
[Typescript-url]: https://www.typescriptlang.org/
[Tauri-shield]: https://img.shields.io/badge/-Tauri-black.svg?style=for-the-badge&logo=tauri&colorB=555
[Tauri-url]: https://tauri.studio/
