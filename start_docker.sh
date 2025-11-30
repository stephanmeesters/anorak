podman container rm --force $(podman ps -aqf "name=anorak")

podman run -it -d \
    --name=anorak \
    -e PUID=1000 \
	-e PGID=1000 \
    -p 9341:9341 \
    -e JACKETT_URL=http://192.168.2.10:9117/api/v2.0/indexers/all/results/torznab \
    -e JACKETT_APIKEY=qjr3cgynmdxzlgwpdjv3x5pwxohzx3zi \
    -e TRANSMISSION_URL=http://192.168.2.10:9091/transmission/rpc \
    stephanm123/anorak:latest
