FROM voidlinux/voidlinux-musl

COPY entrypoint.sh /entrypoint.sh

RUN xbps-install -Sy base-devel curl bash cross-i686-linux-musl cross-aarch64-linux-musl cross-arm-linux-musleabi cross-arm-linux-musleabihf

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    curl -s https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup target add i686-unknown-linux-musl; \
    rustup target add aarch64-unknown-linux-musl; \
    rustup target add armv7-unknown-linux-musleabi; \
    rustup target add armv7-unknown-linux-musleabihf; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    chmod +x /entrypoint.sh;

ENTRYPOINT [ "/entrypoint.sh" ]