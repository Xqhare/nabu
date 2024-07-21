# Nabu

> [!warning]
> This is a hobby project. It is not intended nor ready to be used in production.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` files are going to be my attempt at a general purpose file format, with Nabu as a small wrapper around it, giving the opportunity for downstream projects to create their own files suiting their needs.
I also plan to try out the `Features` flag for the first time, and use that to extend the usability of `.xff` files for a wide range of use-cases.
The only one of these features that is planned, is a key-value store, but I am sure more will come to mind in due time.

## `.xff` specifications
All specifications are in the `specifications` directory. 
V0 can be found [here](specifications/v0.md).

