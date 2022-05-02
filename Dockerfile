# rust official image for rust 1.60.0
FROM rust:1.60 as builder

# use ADD for entire dir
RUN mkdir backend
COPY ./app/backend ./backend
WORKDIR backend

RUN cargo build

EXPOSE 8080

CMD ["cargo", "run"]
