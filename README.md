# i3-wsman-rs

i3-wsman-rs is a command-line tool designed to manage i3 workspaces. It provides functionalities to focus, create, and retrieve workspaces, and includes a Polybar module.

## Installation

None yet.

## Usage

```
Usage: i3-wsman-rs <command> <...args>
```

### Commands

#### `i3-wsman-rs next [create|loop] [...constraints]`

Focuses on the next workspace.

- **Options:**
  - `create`: Optional: If there is no next workspace, create it
  - `...constraints`: Defaults to 'output'. See `get-workspaces`.

#### `i3-wsman-rs adjacent <direction>`

Creates a new workspace next to the current workspace.

- **Directions:** right, left

#### `i3-wsman-rs get-workspaces [...constraints]`

Returns workspaces matching the constraints. Constraints are optional. If none are provided, all workspaces are returned.

- **Constraints:**
  - `focused`: Focused Workspace
  - `visible`: Visible Workspaces
  - `hidden`: Hidden Workspaces
  - `group`: Workspaces apart of the active Group
  - `output`: Workspaces on the output (Expects MONITOR env variable to be set)
  - `output=xyz`: Workspaces on the output xyz

For instance, to get all hidden workspaces on the current monitor:

```
i3-wsman-rs get-workspaces hidden output
```


#### `i3-wsman-rs polybar`

The i3 Workspace Manager Polybar module. To use, add the following to your polybar config.ini:

```ini
[module/i3wsm]
type = custom/ipc
hook-0 = i3-wsman-rs polybar
initial = 1
```

