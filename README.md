# clical

## Features

This app allows for almost anythig you would need when dealing with todos:
- Create, edit, and delete tasks
- Add links to tasks
- Add due dates to tasks
- Add repeating tasks
- Add notes to tasks
- Add tasks to groups (e.g. work, personal, etc.)

## Upcoming features

 - ICal integration
 - Calendar view in your terminal
 - Calendar events to tasks
 - Autocomplete and shortcuts for group management when creating notes
 - Hopefully a method of syncing notes across different machines (with the likes of Syncthing or something similar)

## How to use?

You can run the TUI by executing `clical` anywhere in your terminal. To use the CLI, you can start by running `clical --help`:

```
$ clical --help
A CLI and TUI for your todos

Usage: clical <COMMAND>

Commands:
  ls        Lists all the tasks
  add       Adds a task to your todos
  delete    Deletes a task from your todos
  complete  Marks a task as complete or incomplete
  config    Sets default configurations
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For example:

```
$ clical ls --format json --date-filter today     
[{"id":108,"name":"LF112 Homework","date":"2023-03-16T23:59:59-04:00","repeats":{"DaysOfWeek":["Sunday","Tuesday","Thursday"]},"group":"School","description":null,"url":"https://google.com","complete":false},{"id":114,"name":"LF112 Async Thursday","date":"2023-03-16T23:59:59-04:00","repeats":"Weekly","group":"School","description":null,"url":"https://google.com","complete":false},{"id":107,"name":"EN221 Recitation","date":"2023-03-16T23:59:59-04:00","repeats":{"DaysOfWeek":["Tuesday","Thursday"]},"group":"School","description":null,"url":"https://google.com","complete":false}]
```

## Installation

Use rusts package manger to install clical.

```
cargo install clical
```

## Dependencies

This tool doesn't have any mandatory dependencies. However, it looks much better if you install [Nerd Fonts](https://www.nerdfonts.com/) for better icons. If you don't want to do so, you can always use your own icons or change them for plain text, like `[ ]` for an incomplete task and `[x]` for a complete task.

## [Documentation](https://github.com/danimelchor/clical/blob/main/DOCUMENTATION.md)

## Config

The config file can be found in:
- Unix: `~/.config/clical/settings.json`
- Windows: `C:\Users\<user>\AppData\Roaming\clical\settings.json`

There are some pre-built commands you can run to change the configuration. For example, you can change the keybindings to `vi` mode by running:

```
clical config --mode vi
```

You can also enable special icons by running:

```
clical config --icons special
```

For all the configuration options, run:

```
clical config help
```

Optionally, you can change the default configuration by editing the files directly. The default config is the following:

```
clical config --show
```

```json
{
  "date_formats": {
    "display_date_format": "%a %b %-d",
    "display_datetime_format": "%a %b %-d at %-H:%M",
    "input_date_format": "%d-%m-%Y",
    "input_date_hint": "DD-MM-YYYY",
    "input_datetime_format": "%d-%m-%Y %H:%M",
    "input_datetime_hint": "DD-MM-YYYY HH:MM"
  },
  "show_complete": true,
  "current_group": null,
  "icons": {
    "complete": "[x]",
    "incomplete": "[ ]",
    "repeats": "[r]"
  },
  "colors": {
    "primary_color": "LightGreen",
    "secondary_color": "LightYellow",
    "accent_color": "LightBlue"
  },
  "keybindings": {
    "quit": "q",
    "down": "Down",
    "up": "Up",
    "complete_task": "Space",
    "toggle_completed_tasks": "h",
    "delete_task": "Delete",
    "new_task": "n",
    "edit_task": "e",
    "save_changes": "Enter",
    "enter_insert_mode": "i",
    "enter_normal_mode": "Esc",
    "go_back": "Esc",
    "open_link": "Enter",
    "next_group": "Right",
    "prev_group": "Left"
  }
}
```

## Key Bindings

All key bindings can be modified in the config file. The defaults have been chosen to mimic vim movements as best as possible. Feel free to modify them to your liking!

**List of tasks panel**

| Key Bindings | Description |
| -------- | ---------- | 
| `q` | Quits the application | 
| `Down` | Moves down one task |
| `Up` | Moves up one task |
| `Space` | Marks the task as completed | 
| `h` | Toggles hiding completed tasks |
| `d` | Deletes the selected task forever|
| `n` | Opens the new task page |
| `e` | Focuses the task editing panel |
| `Enter` | If the task has an associated link, it opens it in your preferred browser |
| `Right` | Select next group |
| `Left` | Select previous group |

**Editing/new task panel**

This panel has two modes (similar to vim). When you are in insert mode, you can modify the fields to edit or create a task. When you are in normal mode, you can move around the fields, save the tasks, go back, or quit.

*Normal mode*

| Key Bindings | Description |
| -------- | ---------- |
| `q` | Quit the application |
| `Down` | Move down to the next field |
| `Up` | Move up to the previous field |
| `i` | Enter insert mode |
| `Esc` | Go back to the list of tasks panel |
| `Enter` | Save changes or add the new task |

*Insert mode*

| Key Bindings | Description |
| -------- | ---------- |
| `Esc` | Exit insert mode / go back to normal mode |

## Why create a derivative work of todui?

Because the original app seems to be mostly focused on notes only. I (hrrs01) want to be able to check calendar events quickly from my terminal as well, and create todos based on that.
I also want to have a productivity app which i can safely connect my work email and/or calendar into, without being afraid of sharing my data with third parties.
And jokingly, because all developers need to create their own productivity app somewhere in their career. 

