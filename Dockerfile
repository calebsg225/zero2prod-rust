# ./Dockerfile

# use the latest Rust stable release as the base image
FROM rust:1.86

# use `/app` as the main directory. `/app` is created if it does not exist
WORKDIR /app

# copy all files from working environment to our docker image
COPY . .

# foce sqlx to look at saved metadata
ENV SQLX_OFFLINE=true

# build the binary
RUN cargo build --release

# when `docker run` is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]
