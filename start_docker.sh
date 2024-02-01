docker container rm --force $(docker ps -aqf "name=anorak")
docker image rm anorak:latest -f
docker system prune -f
sh build_docker.sh

docker run -it -d \
    --name=anorak \
    -e PUID=1000 \
	-e PGID=1000 \
    -p 9341:9341\
    anorak:latest
