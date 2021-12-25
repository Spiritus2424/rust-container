FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

ARG USER=vscode

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install --no-install-recommends sudo usbutils git \
    gdb-multiarch openocd qemu-system-arm

RUN cargo install cargo-binutils cargo-generate svd2rust form && \
    rustup component add llvm-tools-preview rustfmt clippy && \
    rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi \
    thumbv7em-none-eabihf thumbv8m.base-none-eabi thumbv8m.main-none-eabi thumbv8m.main-none-eabihf

RUN mkdir -p /dev/bus/usb && \
    usermod -aG plugdev $USER

RUN mkdir -p /usr/local/cargo && \
    chown -R $USER:$USER /usr/local/cargo


USER $USER
