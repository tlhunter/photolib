# `photolib`

An opinionated tool for managing a library of photographs taken by digital cameras.

## Goals

- Always handle pairs of photos, such as `DSC123.JPG` and `DSC123.RAW`, in tandem
- Provide mechanisms for importing photos to the library from a memory card
- Should work on Linux and macOS desktops, and Linux servers
- Probably won't provide a GUI but should be easily shelled out for automation purposes
- Configuration files follow [XDG guidelines](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- Work with RAW files from all of the major camera manufacturers
- Support extracting metadata from RAW files and Darktable files

## Progress

- Nothing works yet

## Opinionated Library Hierarchy

- A given filesystem can have multiple library directories.
- Paths to library locations are stored in a global config file.
- A library can contain multiple collection directories, which can have arbitrary names.
- A collection can contain multiple shoot directories, which should follow a `YYYY-MM-DD Arbitrary Name` pattern.
- Shoot directories can contain multiple photos.
- Photos consist of one or more files which share a common prefix.

Here's an example of a filesystem layout:

```
~/.config/photolib/libraries.json
~/Photos/
~/Photos/Side Projects/
~/Photos/Headshots/
~/Photos/Headshots/2024-06-18 XYZ Corp/
~/Photos/Headshots/2024-06-18 XYZ Corp/DSC001.JPG
~/Photos/Headshots/2024-06-18 XYZ Corp/DSC001.ARW
~/Photos/Headshots/2024-06-18 XYZ Corp/DSC001.ARW.xmp
~/Photos/Headshots/2024-06-18 XYZ Corp/DSC001.export.jpg
~/Photos/Headshots/2024-06-18 XYZ Corp/DSC001.insta.jpg
```

## Subcommands

