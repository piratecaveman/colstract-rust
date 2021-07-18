#!/bin/sh
[ "${TERM:-none}" = "linux" ] && \
    printf '%b' '\e]P0{{color0_strip}}
                 \e]P1{{color1_strip}}
                 \e]P2{{color2_strip}}
                 \e]P3{{color3_strip}}
                 \e]P4{{color4_strip}}
                 \e]P5{{color5_strip}}
                 \e]P6{{color6_strip}}
                 \e]P7{{color7_strip}}
                 \e]P8{{color8_strip}}
                 \e]P9{{color9_strip}}
                 \e]PA{{color10_strip}}
                 \e]PB{{color11_strip}}
                 \e]PC{{color12_strip}}
                 \e]PD{{color13_strip}}
                 \e]PE{{color14_strip}}
                 \e]PF{{color15_strip}}
                 \ec'
