FROM rust:latest

WORKDIR /dockerspace
COPY . .
RUN "rustup component add rustfmt"
RUN "curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash"
RUN "cargo binstall cargo-watch -y"
EXPOSE 9021
CMD [" cargo watch -x check -x test -x fmt -x run --ignore 'content/*'"]
