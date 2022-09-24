# Taken from https://github.com/JoshKeegan/docker-scratch-rust
# By default rustc will only compile with dynamic linking, adding an external runtime dependency on glibc.
#   MUSL supports static though (with some other caveats that don't matter for a simple hello-world app), 
#   so base off of an image with that installed & set up for us and we'll use that build target with the 
#   static flag for a really slim from scratch image.
FROM docker.io/ekidd/rust-musl-builder AS build

WORKDIR /app
COPY --chown=rust:rust . .
RUN cargo install --path .

FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/blockies-service /blockies-service
CMD ["/blockies-service"]
