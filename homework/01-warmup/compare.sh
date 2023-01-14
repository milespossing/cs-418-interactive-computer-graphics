#!/bin/bash
docker run -v /Users/miles/repos/cs-418-interactive-computer-graphics/homework/01-warmup:/pwd -w /pwd --entrypoint= -it --rm dpokidov/imagemagick ./container-compare.sh $@

