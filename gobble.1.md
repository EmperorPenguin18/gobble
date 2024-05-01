% GOBBLE(1) gobble 1.3
% Sebastien MacDougall-Landry
% August 2023

# NAME
gobble - hide your current window while using an external program

# SYNOPSIS
**gobble** [*OPTIONS*] *CMD* ...

# DESCRIPTION
**gobble** uses libxcb to find the currently focused window, then unmaps it. Whatever arguments (other than flags) are passed to it will be interpreted as a command & its arguments and will execute it. When that finishes (or fails) the original window will be remapped.

# OPTIONS
**-h**
: Displays a friendly help message.

**-v**
: Displays the software version.

**-o**
: Uses overlap mode which overlays the new window ontop of the old one instead of unmapping it (useful in floating window managers).

# EXAMPLES
**gobble mpv test.mp4**
: Hides the terminal and opens test.mp4 in mpv (video player program).

**gobble -o mpv test.mp4**
: Opens test.mp4 in mpv and positions it ontop of the terminal.

**gobble -v**
: Displays the software version and exits.

**gobble -h**
: Displays the friendly help message and exits.

# EXIT VALUES
Gobble will exit with whatever code the passed command exited with, or if nothing is passed then 0.


# ISSUES
Only works when using the X11 display server.

Programs that fork immediately (for example as part of a daemon) don't behave right with gobble.

# COPYRIGHT
Copyright © 2021-2024 Sebastien MacDougall-Landry. License GPLv3+: GNU GPL version 3 or later . This is free software: you are free to change and redistribute it. There is NO WARRANTY, to the extent permitted by law.
