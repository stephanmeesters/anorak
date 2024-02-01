docker run -it -d \
    --name=anorak \
    -e PUID=1000 \
	-e PGID=1000 \
    -p 9341:9341\
    anorak:latest
