# Version lock
FROM node:18.9.0-buster-slim AS node_version

# Deps install
FROM node_version AS build_deps

WORKDIR /frontend-build

COPY package.json .
COPY package-lock.json .

RUN npm install

# Build
FROM build_deps AS build

COPY . .

RUN npm run build

RUN mkdir /frontend-release && \
    cp ./dist/* /frontend-release && \
    cp ./pub/* /frontend-release && \
    cp nginx.conf /frontend-release

# Runtime
FROM nginx:latest as runtime

COPY --from=build /frontend-release /frontend-pub

RUN mv /frontend-pub/nginx.conf /etc/nginx/conf.d/default.conf
