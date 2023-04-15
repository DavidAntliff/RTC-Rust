#!/bin/bash
# Calculate the difference image between two images of the same size
# http://www.imagemagick.org/discourse-server/viewtopic.php?p=55272&sid=706927510e4abd061781620910688933#p55272

set -o nounset
set -o errexit

SRC_IMAGE0=$1
SRC_IMAGE1=$2
DIFF_IMAGE=${3:-diff.png}

convert "$SRC_IMAGE0" "$SRC_IMAGE1" \
    \( -clone 0 -clone 1 -compose difference -composite -threshold 0 \) \
    -delete 1 -alpha off -compose copy_opacity -composite -trim \
    "$DIFF_IMAGE"

feh "$DIFF_IMAGE"

