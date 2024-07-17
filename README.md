# Nabu

> [!warning]
> This is a hobby project. It is not intended to be used in production.

The overarching goal of this project is to create a library that can be used to build, read and write .xff files.
.xff files are going to be my attempt at a general purpose file format, with Nabu as a small wrapper around it, giving the opportunity for downstream projects to create their own files suiting their needs.

## .xff specification
.xff should be able to hold any arbitrary data, making some kind of escape mechanism paramount.
I have chosen ASCII encoding, specifically Windows-1252 as the data representation in binary. Mainly because I wanted to work with it.

### Command characters used by .xff
`DLE`, or Data Link Escape is the ideal general escape character inside the ASCII encoding I chose. 

Any data following `DLE`, until another `DLE` is encountered, is considered part of the data-stream. This should make it possible to save any kind of data inside, be it media or documents.
The data inside the escape should be stored as-is, and should not be decoded (e.g. `[u8]`), meaning that inside it could be UTF-8 encoded or JPEG encoded for example.
Here the burden to decode and encode the data lies on the downstream project.

`STX` and `ETX` are the start and end of any text data or numerical data encoded in ASCII.
The text wrapped by it should be returned as a fully decoded `String` or appropriate `Number`.

`EM` is the end of the .xff file.

All other command characters should be returned to the caller, and any non-command characters should error.

![Xff chart](pictures/xff-main-chart.png)
