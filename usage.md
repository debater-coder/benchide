# benchide manual

This editor allows quick access to commands via a command bar, which can be activated using `Ctrl + K`. Below are the 
available commands and their usage.

## Available Commands

### `open <path>`
Opens a new pane with the contents of the file at the relative or absolute path specified. 
**Usage:**
`open folder/myfile.txt`

### `close`
Closes the currently focused pane.

### `save [path]`
Saves the file in the current pane. If a path is provided, a copy of that file is saved at that path; otherwise, it 
will save the file in its current path.
**Usage:**
- Save as a specific file:  
  `save folder/myfile.txt`
- Save the current file:  
  `save`

### `help`
Opens this help manual.

**Note:** You can always reopen this manual by typing the `help` command.
