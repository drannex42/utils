# How to Fix Yakuake in Wayland 

Yakuake has the nasty bug in wayland that if it detects any animation under it, it will start the window in the bottom or center of the screen. This fixes that:

Set up a window rule for org.kde.yakuake and add the property "Initial placement" and set the value to "Force" and "In Top-Left Corner"
