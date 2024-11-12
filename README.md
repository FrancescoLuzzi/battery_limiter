# battery_limiter

Configure max battery charge limit.

## Dev setup

```shell
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential
```

## TODO

use zbus crate to create a service that changes the battery configuration as root,
then the gtk gui will talk to it through dbus (using the zbus crate, [source](https://github.com/dbus2/zbus))

## References

- [bat](https://github.com/tshakalekholoane/bat)
