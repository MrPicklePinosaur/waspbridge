
<div align="center">

# waspbridge

companion application for wasp-os

</div>

**waspbridge** is a **wasp-os** desktop client written in **rust**. It is meant to be used with my fork of the **wasp-os** project, found [here](https://github.com/MrPicklePinosaur/wasp-os). Some features include
- Control mpd using the **wasp-os** Music Player app
- Provide weather updates to **wasp-os** using [wttr.in](wttr.in)

## SETTING UP FOR DEVELOPMENT

Install git hooks
```
$ just devsetup
```

Some python dependencies are needed for **wasptool** and **pynus** to work (the underlying scripts that **waspbridge** uses).
```
$ pip install dbus-python
```

