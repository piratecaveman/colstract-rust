# Colstract [Work In Progress]

Colstract is a program that allows you to generate formatted templates from Xresources or from a configuration file.

## Features

Colstract supports formatting of your custom templates in handlebars.js format  
28 commonly used templates are provided built-in

### Custom template format

The following variables can be used in a custom template:  

```handlebars
{{colorname}} - the color name (see below) - ie. background, foreground, color1 etc. - in hex
{{colorname_hex8}} - the color in hex with alpha transparency value
{{colorname_rgb}} - the color in rgb format - eg. "rgb(255,255,255)"
{{colorname_rgba}} - the color in rgba format - eg. "rgba(255,255,255,0.5)"
{{colorname_xrgba}} - the color in a format used in X window system - eg. "00/00/00/ff"
{{colorname_alpha}} - the alpha value of the color - eg, "0.1"
{{colorname_hex_stripped}} - the color in hex but with the leading # stripped - eg. "ffffff"
{{colorname_hex8_stripped}} - the color in hex with transparency but with the leading # stripped - eg. "000000ff"
{{colorname_rgb_stripped}} - the color in rgb with just the bits - eg. "255,255,255"
{{colorname_rgba_stripped}} - the color in rgba with just the bits - eg. "255,255,255,0.1"
{{alpha}} - direct access to background color's alpha value - eg. "0.6"


Available colorname:
 - background
 - foreground
 - cursor
 - color0
 - color1
 - color2
 - color3
 - color4
 - color5
 - color6
 - color7
 - color8
 - color9
 - color10
 - color11
 - color12
 - color13
 - color14
 - color15
```
