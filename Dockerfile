######################################################################
# Copyright (c) 2021 Silvio Clecio (silvioprog) <silvioprog@gmail.com>
#
# SPDX-License-Identifier: MIT
######################################################################

# docker build -t wethr .
# docker run --rm -t wethr

FROM rust as builder
ENV DEBCONF_NOWARNINGS="yes"
RUN apt-get update && apt-get install musl-tools -y && rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . /app
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /app/target/x86_64-unknown-linux-musl/release/wethr

LABEL Maintainer="Silvio Clecio (silvioprog) <silvioprog@gmail.com>"
LABEL Name="wethr"
LABEL Version="1.0.0"

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/wethr .
ENTRYPOINT ["./wethr"]
