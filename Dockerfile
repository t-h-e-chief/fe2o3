# fe3o2 - Rust Demo
#
# VERSION               1.0.0

FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo test --release
RUN cargo build --release

# Now build release docker image.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
EXPOSE 8080
COPY --from=builder  /home/rust/src/target/x86_64-unknown-linux-musl/release/fe3o2 /usr/local/bin/
CMD /usr/local/bin/fe3o2
