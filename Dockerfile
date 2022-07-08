ARG IMAGE_NAME
ARG IMAGE_VERSION

FROM ${IMAGE_NAME}:${IMAGE_VERSION}

WORKDIR /opt/traiter

COPY rust-toolchain.toml .
COPY README.md .
COPY Cargo.toml .
COPY src src
COPY tests tests
