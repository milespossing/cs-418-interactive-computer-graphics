#!/bin/bash

magick compare -fuzz 2% $1 $2 ae.png
magick composite $1 $2 -compose difference rawdiff.png
magick convert rawdiff.png -level 0%,8% diff.png
magick convert +append $2 $1 ae.png rawdiff.png diff.png $3
rm rawdiff.png diff.png ae.png
