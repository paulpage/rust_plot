#!/bin/sh
mpv g.opus &
target/debug/rust_plot data.txt
convert graph.jpg \( -resize 830x280 -background none -rotate -15 out.png -geometry +740+300 \) -composite result.png
sxiv result.png
