# Nabu

> [!warning]
> This is a hobby project. It is not intended nor ready to be used in production.

Nabu is a rust library for reading and writing `.xff` files.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` files are my attempt at a general purpose file format, with Nabu as the translation layer, giving the opportunity for downstream projects to create their own files suiting their needs.
I am also trying out the `Feature` flags for the first time, to extend the usability of `.xff` files for a wide range of use-cases without needing to roll a bespoke library.

As with all my projects, this documentation contains everything you never wanted to know about `.xff` files, Nabu and how to work with them.
Just read the examples below and have fun.

## 1. Motivation
After finishing [Mawu](https://github.com/Xqhare/mawu), I wanted to dive deeper into file structures and working with bytes directly, instead of `&str` and later `chars` like in Mawu. Around this time I also had my first deep dive on ASCII after rewatching "The Martian" and thus decided on making my own file format.
I wrote v0 of the `.xff` specification in just a few days, and then started working on the implementation.
This library is also a major part of my own tech stack.
As `xff` is meant to be a jack of all trades, it is important that it can be used in a wide range of use-cases.

## 2. Naming
This library's namesake is the ancient Babylonian god Nabu, the god of literacy, rational arts and scribes.
As the inventor of writing, Nabu is a fitting namesake for a tool designed to create and interpret a new form of written data.

## 3. Contents
- [1. Motivation](#1.-Motivation)
- [2. Naming](#2.-Naming)
- [3. Contents](#3.-Contents)
- [4. Roadmap](#4.-Roadmap)
- [5. Implemented Features](#5.-Implemented-Features)
- [6. `.xff` specifications](#6.-.xff-specifications)
- [7. Usage](#7.-Usage)
    - [7.1. Importing](#7.1.-Importing)
    - [7.2. Default Modules](#7.2.-Default-Modules)
        - [7.2.1. Usage of serde](#7.2.1.-Usage-of-serde)
    - [7.3. XffValue](#7.3.-XffValue)
    - [7.4. Feature Flags](#7.4.-Feature-Flags)
        - [7.4.1. Logging Wizard](#7.4.1.-Logging-Wizard)
        - [7.4.2. Config Wizard](#7.4.2.-Config-Wizard)
        - [7.4.3. Key Value Store](#7.4.3.-Key-Value-Store)
            - [7.4.3.1. Key Value Core](#7.4.3.1.-Key-Value-Core)
                - [7.4.3.1.1. Key Value Core Usage](#7.4.3.1.1.-Key-Value-Core-Usage)
            - [7.4.3.2. Key Value Store](#7.4.3.2.-Key-Value-Store)
                - [7.4.3.2.1. Key Value Store Usage](#7.4.3.2.1.-Key-Value-Store-Usage)


