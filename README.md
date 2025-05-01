# swayidle-switch
## Description
Sometimes I want my computer to lock itself automatically after five minutes. Sometimes I want to watch a 10 minute YouTube video. The solution? Swayidle-switch! This will look in your `~/.config/sway/config` file, extract the Sway Idle command that Sway uses by default, and respectively kill or start it. 

## Setup
You must preface a line before your Sway Idle command with `#! SWAYIDLE START !#` and a line after your Sway Idle command with `#! SWAYIDLE END !#` so that it can be parsed by this program. 

For example:
```
#! SWAYIDLE START !#
exec swayidle -w \
          timeout 300 'exec playerctl pause' \
          timeout 600 'exec systemctl suspend' \
          timeout 1200 'exec systemctl hibernate'
#! SWAYIDLE END !#
```

## Use
Either run `swayidle-switch` from your terminal, or, as I find to be much more convenient, bind `swayidle-switch` to a hotkey, like so:
```
bindsym $mod+Shift+t exec swayidle-switch
```

Either will work though!
