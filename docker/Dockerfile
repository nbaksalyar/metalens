FROM fedora:36

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN dnf install -y llvm llvm-libs llvm-devel llvm-static kernel kernel-devel make elfutils-libelf-devel zstd
RUN llvm-config --version | grep -q '^14'
RUN curl https://sh.rustup.rs -sSf > rustup.sh \
    && sh rustup.sh -y --default-toolchain stable --profile minimal --no-modify-path \
    && rm -f rustup.sh \
    && rustup component add rustfmt \
    && rustup --version \
    && cargo -vV \
    && rustc -vV
RUN dnf install -y clang-devel libffi-devel
# update crates.io index - workaround for https://github.com/rust-lang/cargo/issues/3377
RUN cargo install empty-library; exit 0

WORKDIR /build
