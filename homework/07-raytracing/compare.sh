#!/bin/bash
# Evaluates for all given results. uses Mine - Expected - Comparison order
for file in ./results/*.png; do
  name=$(basename $file)
  echo "$name"
  A=results/$name
  B=features/$name
  magick compare -fuzz 2% $A $B ae.png
  magick composite $A $B -compose difference rawdiff.png
  magick convert rawdiff.png -level 0%,8% diff.png
  magick convert +append $A $B ae.png rawdiff.png diff.png comparisons/$name
  rm rawdiff.png diff.png ae.png
done

