#!/bin/bash

docker run --rm -v $(pwd):/pwd -w /pwd image-compare $1
