# holocene

Takes advantage of the `chrono` crate to provide a GNU `date` like interface for convert dates to the Holocene calendar.

## Known issues

* Timezone support is not implemented yet. UTC is assumed.
* Years must be zero padded to 4 digits. E.g. `0001` instead of `1`.

## Example usage

View the built-in help message.
```shell
holocene --help
```

Get the current holocene date and time (UTC.)
```shell
holocene
```

Get the current holocene date and time (UTC) with custom formatting.
```shell
holocene "+%m-%d-%Y %H:%M:%S %z %Z %E"
```

Get the holocene date for a specific date.
```shell
holocene -d "12/31/1970"
```
```shell
holocene -d "1970/12/31"
```

Get the holocene date for a specific date and time (UTC.)
```shell
holocene -d "12/31/1970 23:59:59"
```
```shell
holocene -d "1970/12/31 23:59:59"
```

Get a holocene date for a specific BCE date.
```shell
holocene -d "12/31/0337 BCE"
```

```shell
holocene -d "0337/12/31 12:01:57 BCE"
```

Get the holocene date for a specific date and time (UTC) with custom formatting.
```shell
holocene -d "12/31/1970 23:59:59" "+%m-%d-%Y %H:%M:%S %z %Z %E"
```