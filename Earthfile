VERSION 0.8

deps:
    FROM node:latest
    WORKDIR /packages/docreader

build:
    FROM +deps
    COPY . .
    SAVE ARTIFACT .