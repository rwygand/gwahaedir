####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
RUN apt install libssl-dev

# Create appuser
ENV USER=gwahaedir
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /gwahaedir

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /gwahaedir

# Copy our build
COPY --from=builder /gwahaedir/target/x86_64-unknown-linux-musl/release/gwahaedir ./

# Use an unprivileged user.
USER gwahaedir:gwahaedir

CMD ["/gwahaedir/gwahaedir"]