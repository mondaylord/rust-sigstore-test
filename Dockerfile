# Stage 1: Build the application
FROM rust:1.78 as builder

WORKDIR /usr/src/app
COPY . .
# The build command from the original workflow
RUN cargo build --release

# Stage 2: Create the final, minimal image
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/app/target/release/rust-sigstore-test /usr/local/bin/rust-sigstore-test
CMD ["/usr/local/bin/rust-sigstore-test"]