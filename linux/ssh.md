
# SSH 

## SSH-ASKPASS

[I know this is several years old](https://github.com/nerves-project/nerves/issues/33), but for anyone else who may end up googling for a solution to `sudo: unable to run /usr/bin/ssh-askpass: No such file or directory`

You will need to create a symbolic link between ssh-askpass and your existing askpass installation.

```bash
ln -s $SSH_ASKPASS /usr/bin/ssh-askpass

# or 

ln -s /location/to/askpass /usr/bin/ssh-askpass
```

If the above didn't work, check to see if you have $SSH_ASKPASS set by running echo $SSH_ASKPASS in your terminal.

If you don't, then you need to either install ssh-askpass
- Ubuntu: sudo apt get ssh-askpass
- Fedora: sudo dnf install gnome-ssh-askpass

Now rerun the above after those are installed. 
