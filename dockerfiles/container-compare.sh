#!/bin/bash

magick compare -fuzz 2% $1.png $1.expected.png ae.png
magick composite $1.png $1.expected.png -compose difference rawdiff.png
magick convert rawdiff.png -level 0%,8% diff.png
magick convert +append $1.expected.png $1.png ae.png rawdiff.png diff.png $1.result.png
rm rawdiff.png diff.png ae.png
