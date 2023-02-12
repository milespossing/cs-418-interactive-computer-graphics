#!/bin/bash

cargo r -- $1.txt
docker run --rm -v $(pwd):/pwd -w /pwd image-compare $1
open $1.result.png
