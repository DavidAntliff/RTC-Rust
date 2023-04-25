#!/bin/bash
#
# Save a PPM image to a timestamped PNG file in the images/ directory.
#
# E.g.
#
#    save.sh chapter8 image2.ppm   # creates images/chapter8_<timestamp>.png from image2.ppm
#
#    save.sh chapter8              # creates images/chapter8_<timestamp>.png from default image.ppm
#
#    save.sh                       # creates images/<timestamp>.png
#

set -o nounset
set -o errexit

TIMESTAMP=$(date "+%Y%m%d_%H%M%S")

PREFIX=${1:-}
SRC_IMAGE=${2:-image.ppm}
DST_DIR=${3:-images}

if [[ -z $PREFIX ]]
then
    DEST_IMAGE="${DST_DIR}/${TIMESTAMP}.png"
else
    DEST_IMAGE="${DST_DIR}/${PREFIX}_${TIMESTAMP}.png"
fi

mkdir -p $(dirname "$DEST_IMAGE")
convert "$SRC_IMAGE" "$DEST_IMAGE"

echo "$SRC_IMAGE -> $DEST_IMAGE"
