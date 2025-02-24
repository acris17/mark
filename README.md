# mark
Manage file system paths like browser bookmarks.

## Examples

```$ cd $(mark get currentProject)```

```$ mark open currentProject```

```$ mark edit```

```$ mark help```

## Build
1. Run make in the terminal.

```$ make```

2. Ensure the program is in release directory.

```./target/release/mark```


## Configuration
Run ```$ mark edit``` to edit the list of bookmarks. An example toml file should look like this:
```
currentProject="~/Development/my-project"
resume="~/Documents/resume.pdf"
```
