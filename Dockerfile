FROM rust:latest as build

RUN USER=root cargo new --bin known
WORKDIR ./known
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./migration ./migration
COPY ./common ./common
COPY ./entity ./entity

RUN USER=root cargo new --bin app
COPY ./app/Cargo.toml ./app/Cargo.toml

# get dependencies for each manifest member
RUN cargo build --bin app --release

# remove main.rs or any initial rust code
RUN rm ./app/src/*.rs

# move the src code
COPY ./app/src ./app/src
COPY .env .env

# remove compiled code relating to this project
RUN rm ./target/release/deps/app*

RUN cargo build --release

# Use an image that doesn't have Rust tools included
FROM debian:bullseye

EXPOSE 3005

COPY --from=build /known/target/release/app .

CMD ["./app"]

# sources
# https://betterprogramming.pub/how-to-deploy-a-rust-web-server-to-heroku-using-axum-docker-and-github-actions-6cddb442ea7e
# https://kerkour.com/rust-small-docker-image
# https://www.youtube.com/watch?v=QCktBeTkOjA