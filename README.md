# bifes

Lists files and directories larger than a specified threshold in the specified directory.

**Usage**: `bifes <measure> <threshold> <target>`
The measure can be `-k`, `-m`, `-g`, `-t` for k, mega, giga, or terabytes. If not specified, assumed to be `-m`. If specified, it is also necessary to indicate the threshold for file size.
The threshold is the minimum number of k, mega, giga, or terabytes that a file or directory must have in order to be listed. This can only be specified with a measure. If not specified, assume to be 1.
The `target` is the target directory whose elements will be listed. The sizes of directories within the target account for all their sub-directories, recursively. If not specified, uses the current directory.
