# Guild Wars 2 Build Bank
![banner](/docs/banner.png)

A minimalist tool to store build templates from the game Guild Wars 2.

## Demo
You can watch a video demo [here on Youtube](https://www.youtube.com/watch?v=PRpPOlQEKt8)

## Getting it

### Prebuilt binaries
Get one of the prebuilt binaries from the [releases](https://github.com/Aelto/gw2-build-bank/releases)

### Compiling from the source
 1. Start by installing the Rust compiler using the instructions related to your operating system
 2. Download the source code from the [releases](https://github.com/Aelto/gw2-build-bank/releases), using the `Source code (zip)` links or from the git repository directly
 3. Decompress the source and open a terminal in the new decompressed directory for the project
 4. Run the following command: `cargo build --release`
 5. Head into `target/release` and get the binary/executable you just generated. It is named `gw2-build-bank`

## Minimalist
The tool doesn't try to do a hundred things, no build preview, no gear storage either and no automatic template swapping.

Instead it uses your clipboard and the native Guild Wars 2 feature to copy/paste templates from the clipboard.

## Lightweight
The tool was made to be able to run in background without crippling the host' machine. During my tests, which may vary depending on the machine, the application uses the following resources:
 - 0% cpu
 - 18mb of ram
 - 0% disk
 - 0% network
 - 0% gpu

Disk size is also important. but to make the application pleasing to the eye, images were used and so size has increased a bit.
 - the binary/executable itself is approximately 10mb large
 - once run, the application unbundles a few necessary files for the UI for a total of 45mb approximately, binary included
 - each build, depending on description you write weights at minimum less than 1 kilobyte.

## Keybinds
The application is meant to run in the background, and so it listens for a global keybind. Whenever `LEFT-CTRL + M` is used, the application is pushed to the first layer in front of all the other running programs.

> This keybind is hardcoded and cannot be changed

## Known issues
The [UI library i used](https://docs.microsoft.com/fr-fr/microsoft-edge/webview2/) is still being worked on by Microsoft and has one little bug i noticed. If you move the window the dropdown lists will still appear at the previous location of the application, resulting in a floating dropdown list.

It's a rather small issue that can be ignored if you don't move the window around much.
