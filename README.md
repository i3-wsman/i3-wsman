<p align="center">
  <img src="docs/images/banner-light.png#gh-light-mode-only" alt="i3-wsman" />
  <img src="docs/images/banner-dark.png#gh-dark-mode-only" alt="i3-wsman" />
</p>

<p align="center">
  Create, reorder, group, and focus workspaces fast and easily in i3.
</p>

<p align="center">
  <a href="https://www.buymeacoffee.com/dalrymple" target="_blank"><img src="https://img.buymeacoffee.com/button-api/?text=Support This Project&emoji=&slug=dalrymple&button_colour=BD5FFF&font_colour=ffffff&font_family=Lato&outline_colour=000000&coffee_colour=FFDD00" /></a>
</p>

## Features

### Focus Mode: Eliminate Distractions

**Enable Focus Mode**: Use groups and focus mode to hide workspaces so you can focus.

<img src="docs/images/focus-mode.webp" alt="i3-wsman enables distrction free" />

**Stay Focused**, by limiting navigation to workspaces in the current focus mode.

<img src="docs/images/focus-switch-ws.webp" alt="i3-wsman allows you to stay focused" />

**Allow Important Distractions Through**: Urgent workspaces will peek through, so you don't miss anything.

<img src="docs/images/urgent-peek.webp" alt="i3-wsman allows important distractions through" />

**Multitask**: _Right-click_ to focus multiple groups at a time.

<img src="docs/images/focus-multitask.webp" alt="i3-wsman allows you to multitask" />


### Organize and Create Workspaces

**No More Looping**: Creating a workspace is now as easy as going to the next workspace! Workspaces are created automatically.

<img src="docs/images/create-next.webp" alt="i3-wsman creates new workspaces" />


**Organize Workspaces**: Reorder workspaces left and right to optimize workflow.

<img src="docs/images/focus-reorder.webp" alt="i3-wsman allows reordering workspaces" />

**Create Adjacent Workspaces**: Squeeze in a new workspace to the left or right, so you don't have to reorder!

<img src="docs/images/create-adjacent.webp" alt="i3-wsman allows creating adjacent workspaces" />


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

## Getting Started

This project is **_brand new_**. These steps will become easier and more automated soon. Star and watch this repo for updates!

### Build Manually

1. Clone this repo
2. Build the project with `cargo build --release`

### Install

Distro-specific packages will be coming soon. Star and watch this repo for updates!

- Install `i3-wsm` by placing it in your path

### Configure

1. Copy [`examples/i3-wsman.toml`](examples/i3-wsman.toml) to `~/.config/i3/i3-wsman.toml` (CLI Configurator coming soon)
2. Update your `~/.config/i3/config`
    - Be sure to add `exec --no-startup-id "i3-wsman polybar watch"`
3. Update your `~/.config/polybar/config.ini`

<details>
<summary><b>Click to see an example i3/config</b></summary>

```shell
# switch to workspace
bindsym $mod+1 exec --no-startup-id "i3-wsman goto 1"
bindsym $mod+2 exec --no-startup-id "i3-wsman goto 2"
bindsym $mod+3 exec --no-startup-id "i3-wsman goto 3"
bindsym $mod+4 exec --no-startup-id "i3-wsman goto 4"
bindsym $mod+5 exec --no-startup-id "i3-wsman goto 5"
bindsym $mod+6 exec --no-startup-id "i3-wsman goto 6"
bindsym $mod+7 exec --no-startup-id "i3-wsman goto 7"
bindsym $mod+8 exec --no-startup-id "i3-wsman goto 8"
bindsym $mod+9 exec --no-startup-id "i3-wsman goto 9"
bindsym $mod+0 exec --no-startup-id "i3-wsman goto 10"

# Left/Right Navigation
bindsym $mod+Ctrl+Left exec --no-startup-id "i3-wsman prev create group nogroup output"
bindsym $mod+Ctrl+Right exec --no-startup-id "i3-wsman next create group nogroup output"

# Reorder Workspace
bindsym $mod+Shift+Ctrl+Left exec --no-startup-id "i3-wsman reorder left"
bindsym $mod+Shift+Ctrl+Right exec --no-startup-id "i3-wsman reorder right"

# Create adjacent workspace
bindsym $mod+Mod1+Left exec --no-startup-id "i3-wsman adjacent left"
bindsym $mod+Mod1+Right exec --no-startup-id "i3-wsman adjacent right"

# Assign workspace to group
bindsym $mod+Shift+g exec --no-startup-id i3-input -F 'exec --no-startup-id "i3-wsman group assign %s"' -P 'Group: '

# Start the i3-wsman watcher
exec --no-startup-id "i3-wsman polybar watch"
```

</details>


<details>
<summary><b>Click to see an example polybar/config.ini</b></summary>

```ini
[bar/my-bar]
; ...
modules-left = i3wsm-groups i3wsm-workspaces i3wsm-toggle-hidden

[module/i3wsm-groups]
type = custom/ipc
hook-0 = i3-wsman polybar module-groups no-all
initial = 1
format = <label>
format-font = 3

[module/i3wsm-toggle-hidden]
type = custom/ipc
hook-0 = i3-wsman polybar module-toggle-hidden
initial = 1
format = <label>
format-font = 3

[module/i3wsm-workspaces]
type = custom/ipc
hook-0 = i3-wsman polybar module-workspaces
initial = 1
format = <label>
format-font = 3
```

</details>

