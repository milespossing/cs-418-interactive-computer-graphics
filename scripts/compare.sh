#!/bin/bash
docker run -v $(pwd):/pwd -w /pwd --entrypoint= -it --rm dpokidov/imagemagick ./container-compare.sh $@

