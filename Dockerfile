# rust official image for rust 1.59.0
FROM rust:1.59 as builder

# create dummy for deps caching
RUN cargo new culinary_ocean
WORKDIR /culinary_ocean

# copy deps
COPY ./Cargo.toml ./Cargo.lock ./

# cache deps for later
RUN cargo build --release

# copy src
RUN rm src/main.rs
# use ADD for entire dir
ADD ./src ./src

# compile src
RUN cargo install --path .

CMD ["culinary_ocean"]
