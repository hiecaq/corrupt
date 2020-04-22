corrupt
============================

In short: this tool colors strings input from stdin and outputs them to stdout.

Color theme is defined with RON, and what to color is defined based on regexps. Both of which are user-defined input (currently only with cli arguments, config-file-based support is planned).

Try this:
```sh
ping 192.168.1.1 | corrupt 'bytes?|time' '[0-9]{1,3}(\.[0-9]{1,3}){3,3}' -s '[Fg(Red), Bg(Blue)]' '[Fg(Blue), Attr(Underlined)]'
```
