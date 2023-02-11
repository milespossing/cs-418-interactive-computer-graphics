FROM dpokidov/imagemagick

WORKDIR /scripts
COPY container-compare.sh .
WORKDIR /pwd
ENTRYPOINT ["/scripts/container-compare.sh"]
