# `photolib`

An opinionated tool for managing a library of photographs taken by digital cameras.

Note that I'm teaching myself Rust while working on this project. It may never reach production-ready status.

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
~/.config/photolib/config.ini
~/Photos/
~/Photos/.photolib.db
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

- `photolib init`: Initialize the configuration file
- `photolib new <path> (<alias>)`: Create a new library
- `photolib check <namespace?>`: Check for library errors (e.g. mismatched dates)
- `photolib import <path>`: Copy or move files to a library
- `photolib dupes <namespace?>`: Check for photos duplicated across different shoots
- `photolib report <namespace?>`: Provide library statistics such as focal length frequency
- `photolib fix`: Prompt user to fix mismatched dates, delete duplicates, etc.
- `photolib list collections`: List all collections for the current library
- `photolib list shoots <collection?>`: List all shoots for library or specified collection
- `photolib list photos <namespace?>`: List all photos (DSC001) for given shoot
- `photolib prune <namespace?> --min-rating=1`: Delete photos below a certain rating

In these commands, `<path>` refers to a filesystem path, `<namespace>` refers to either a library, a library/collection, or a library/collection/shoot.

Some things can be assumed, for example if the current working directory is inside of a photolib library then photolib will assumed the commands are for that library.
Commands which delete files probably shouldn't make such assumptions.

Deletions will send files to trash by default, e.g. by shelling out to `gio trash <filename>` on Linux.