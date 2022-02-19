# Display Managers

## Fedora

How to change display managers on Fedora 30+


```bash
# Find out the type of display manager you have
ls -al /etc/systemd/system/display-manager.service

# Disable the existing one
systemctl disable lightdm (or gdm/sddm)

# Enable the new one
systemctl enable gdm (or lightdm/sdsm)
```
