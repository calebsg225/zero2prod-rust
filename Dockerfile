# ./Dockerfile

# Builder Stage

# use the latest Rust stable release as the base image
FROM rust:1.86.0 AS builder

# use `/app` as the main directory. `/app` is created if it does not exist
WORKDIR /app
# copy all files from working environment to our docker image
COPY . .
# foce sqlx to look at saved metadata. This is required because sqlx checks that all queries are successful at compile time.
ENV SQLX_OFFLINE=true

# build the binary
RUN cargo build --release

# Runtime Stage

FROM debian:bookworm-slim AS runtime

WORKDIR /app

# install OpenSSL and ca-cerificates
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

# copy the compiled binary from the builder environment to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod

# copy over config files
COPY configuration configuration

# use the production configuration
ENV APP_ENVIRONMENT=production

# when `docker run` is executed, launch the binary
ENTRYPOINT [ "./zero2prod" ]
