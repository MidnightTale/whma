# WHMA (Window Hover Mouse Activation)

A lightweight, efficient window manager that automatically activates windows when hovering over them with your mouse cursor for windows 10/11.

## Configuration

Configuration file is located at `~/.config/whma/config.json`:

```json
{
  "delay_ms": 50,
  "cooldown_ms": 300,
  "enabled": true,
  "blacklist": [
    {
      "window_title": "Task Manager",
      "class_name": "",
      "process_name": "Taskmgr.exe"
    }
  ]
}