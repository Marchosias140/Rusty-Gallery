FROM fedora:latest

RUN dnf -y update

COPY wallpaper_gallery /usr/bin

WORKDIR /home

RUN mkdir static

RUN mkdir static/wallpapers

CMD ["wallpaper_gallery"]
