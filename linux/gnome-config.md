# Gnome Config

## Proper Dock 

### Cycle Windows on Dock Click
```
gsettings set org.gnome.shell.extensions.dash-to-dock click-action 'cycle-windows'
```

## Monitor

### Fractional Scaling 

This allows you to have increments of 25% in monitor scaling. 

```
gsettings set org.gnome.mutter experimental-features "['scale-monitor-framebuffer']"
```
