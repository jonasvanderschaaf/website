FROM rust:1.56

# Create empty project
RUN USER=root cargo new --bin server
WORKDIR /server

# Copy manifests
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

# Build the project and its dependencies
RUN cargo build
RUN rm src/*.rs

# Copy project files
COPY src src

# Build the project
RUN rm ./target/debug/deps/webserver*
RUN cargo build

# Copy Rocket.toml and templates
COPY Rocket.toml Rocket.toml

EXPOSE 8000

CMD ["cargo", "run"]
