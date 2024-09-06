# Windows WSL 

## GUI

If using ArchWSL or other distros, then you will need to forward X/Wayland out for GUI applications. In recent versions you need to setup WSLG, with mounting a specific temp file. Thanks to @darkvertex [for the suggestion](https://github.com/microsoft/WSL/issues/6999#issuecomment-2303010704). 

`/etc/tmpfiles.d/wslg.conf`:
```
#  This file is part of the debianisation of systemd.
#
#  systemd is free software; you can redistribute it and/or modify it
#  under the terms of the GNU General Public License as published by
#  the Free Software Foundation; either version 2 of the License, or
#  (at your option) any later version.

# See tmpfiles.d(5) for details

# Type Path           Mode UID  GID  Age Argument
L+     /tmp/.X11-unix -    -    -    -   /mnt/wslg/.X11-unix
```

and then wsl --shutdown and start it again to see the effect.

This results in the symlink being created, and it happens after tmpfs is mounted by systemd.

## VPN 

If you are behind a corporate proxy or VPN you are likely going to need to reroute your network connection through the proxy on your main machine. Thankfully, there is a tool that allows for this. Check out [WSL-VPNKit](https://github.com/sakai135/wsl-vpnkit) for setup and installation. 

## Distros 

### ArchWSL 

ArchWSL is a basic Arch install for WSL, works great, doesn't require direct updates often (since it's arch). I prefer using the scoop install method. 

- [https://github.com/yuk7/ArchWSL](https://github.com/yuk7/ArchWSL) 
