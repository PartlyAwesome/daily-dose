FROM rust:1.88.0 as builder

RUN update-ca-certificates

ENV USER=dailydose
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /dailydose

COPY ./ .

RUN cargo build --release

##
##
##

FROM gcr.io/distroless/cc

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /dailydose

COPY --from=builder /dailydose/target/release/daily-dose ./
COPY config.toml dailydose.mp4 kill.png ./

USER dailydose:dailydose

ENTRYPOINT ["/dailydose/daily-dose"]