<p align="center">
  <img src="docs/images/banner-light.png#gh-light-mode-only" alt="i3-wsman" />
  <img src="docs/images/banner-dark.png#gh-dark-mode-only" alt="i3-wsman" />
</p>

<p align="center">
  Create, reorder, group, and focus workspaces fast and easily in i3.
</p>

## Features

### Focus Mode: Eliminate Distractions

**Enable Focus Mode**: Use groups and focus mode to hide workspaces so you can focus.

<img src="docs/images/focus-mode.webp" alt="i3-wsman enables distrction free" />

**Stay Focused**, by limiting navigation to workspaces in the current focus mode.

<img src="docs/images/focus-switch-ws.webp" alt="i3-wsman enables distrction free" />

**Allow Important Distractions Through**: Urgent workspaces will peek through, so you don't miss anything.

<img src="docs/images/urgent-peek.webp" alt="i3-wsman enables distrction free" />

**Multitask**: _Right-click_ to focus multiple groups at a time.

<img src="docs/images/focus-multitask.webp" alt="i3-wsman enables distrction free" />


### Organize and Create Workspaces

**Organize Workspaces**: Reorder workspaces left and right to optimize workflow.

<img src="docs/images/focus-reorder.webp" alt="i3-wsman enables distrction free" />

**Create Adjacent Workspaces**: Squeeze in a new workspace to the left or right, so you don't have to reorder!

<img src="docs/images/create-adjacent.webp" alt="i3-wsman enables distrction free" />


## Current Features

#### Focus Mode
- Assign workspaces to groups
- Select one or more groups to focus
  - Optional: Auto-focus on nearest workspace
- Multi-monitor support
  - Optional: Per-monitor groups and focus mode

#### Create and Navigate

- Reorder workspaces
- Create adjacent workspace
- Next/Prev Workspace
  - Optional: Create new, loop, or do nothing
- New workspaces inherit the group of the current workspace


## Coming Soon

- Expanded `i3-wsman` configuration
  - New Startup, Create, and Navigation options coming
- Polybar module formatting and styling
- CLI Configurator for config and polybar styling
- Move window/container to next/prev workspace
- Move window/container to new workspace on the left/right
- Auto-assign new workspace based on application

### Future Roadmap

- Polybar animations for actions (fade out workspaces, animation for swapping, etc...)
- New UI for assigning Workspace to Group
- Workspace Picker UI: Preview workspaces Expose-style
- Assign a workspace to multiple groups (maybe?)


## Installation

None yet.

## Usage

```
Usage: i3-wasman <command> <...args>
```

### Commands

#### `i3-wasman next [create|loop] [...constraints]`

Focuses on the next workspace.

- **Options:**
  - `create`: Optional: If there is no next workspace, create it
  - `...constraints`: Defaults to 'output'. See `get-workspaces`.

#### `i3-wasman adjacent <direction>`

Creates a new workspace next to the current workspace.

- **Directions:** right, left

#### `i3-wasman get-workspaces [...constraints]`

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
i3-wasman get-workspaces hidden output
```


#### `i3-wasman polybar`

The i3 Workspace Manager Polybar module. To use, add the following to your polybar config.ini:

```ini
[module/i3wsm]
type = custom/ipc
hook-0 = i3-wasman polybar
initial = 1
```

