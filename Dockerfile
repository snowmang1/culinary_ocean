# rust official image for rust 1.59.0
FROM rust:1.59 as builder

# create working dir
RUN mkdir culinary_ocean
WORKDIR /culinary_ocean

# use ADD for entire dir
COPY ./ ./app

# needed for custom build scripts
RUN cargo install cargo-make

# slim build of rust to just run the server
FROM rust:1.59-slim as server

# copy only backend from builder
RUN cargo new backend
COPY --from=builder ./backend/* ./backend

# simple run
CMD ["cargo", "run"]
