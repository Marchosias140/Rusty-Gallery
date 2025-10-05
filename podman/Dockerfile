FROM fedora:latest

RUN dnf -y update

COPY rusty-gallery /usr/bin

WORKDIR /home

RUN mkdir static

RUN mkdir static/wallpapers

CMD ["rusty-gallery"]
