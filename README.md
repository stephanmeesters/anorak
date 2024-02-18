# Anorak

Anorak is a self-hosted media app for requesting torrents from Jackett and sending them to web based bittorrent servers like Transmission.
Its purpose is to be a general purpose Torrent grabber, as opposed to specialized grabbers such as Sonarr, Radarr and SickChill.

Developed using Rust / Axum / MiniJinja / HTMX

## How to use

- Install Jackett - https://docs.linuxserver.io/images/docker-jackett/
- Install Transmission - https://docs.linuxserver.io/images/docker-transmission/

You can run the Docker container using the following prompt:

```
docker run -it -d \
    --name=anorak \
    -e PUID=1000 \
    -e PGID=1000 \
    -p 9341:9341\
    -e JACKETT_URL=[your-jackett-torznab-url] \
    -e JACKETT_APIKEY=[your-jackett-apikey] \
    -e TRANSMISSION_URL=[your-transmission-URL] \
    --restart unless-stopped \
    stephanm123/anorak:latest
```

- JACKETT_URL is for example `http://192.168.2.10:9117/api/v2.0/indexers/sometorrentprovider/results/torznab`
- JACKETT_APIKEY can be obtained from the Jackett page
- TRANSMISSION_URL is for example `http://192.168.2.10:9091/transmission/rpc`

Currently Anorak does not support a user/password combo for Transmission.

## Development goals

- [x] Face-lift with CSS
- [x] Indicate which torrents are already grabbed
- [ ] Support user/password authentication for Transmission
- [ ] Add other torrent downloaders
- [ ] Add configuration options within the webapp (thus removing the docker params)
- [ ] Indicate free space on the server
- [ ] Indicate number of seeds/peer and other useful info
