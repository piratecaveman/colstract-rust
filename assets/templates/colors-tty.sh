#!/bin/sh
[ "${TERM:-none}" = "linux" ] && \
    printf '%b' '\e]P0{{color0_hex_stripped}}
                 \e]P1{{color1_hex_stripped}}
                 \e]P2{{color2_hex_stripped}}
                 \e]P3{{color3_hex_stripped}}
                 \e]P4{{color4_hex_stripped}}
                 \e]P5{{color5_hex_stripped}}
                 \e]P6{{color6_hex_stripped}}
                 \e]P7{{color7_hex_stripped}}
                 \e]P8{{color8_hex_stripped}}
                 \e]P9{{color9_hex_stripped}}
                 \e]PA{{color10_hex_stripped}}
                 \e]PB{{color11_hex_stripped}}
                 \e]PC{{color12_hex_stripped}}
                 \e]PD{{color13_hex_stripped}}
                 \e]PE{{color14_hex_stripped}}
                 \e]PF{{color15_hex_stripped}}
                 \ec'
