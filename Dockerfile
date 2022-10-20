FROM rust:1.64.0

WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y 
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
