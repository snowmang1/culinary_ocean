# rust official image for rust 1.59.0
FROM rust:1.59

WORKDIR usr/src/culinary_ocean
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["culinary_ocean"]
