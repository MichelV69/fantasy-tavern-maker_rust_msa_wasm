FROM nixos/nixos/nix AS init
WORKDIR /dockerspace
RUN mkdir -p /dockerspace/content/css
RUN mkdir -p /dockerspace/content/images
RUN mkdir -p /dockerspace/content/templates
COPY content/* /dockerspace/content/*
COPY target/debug/fantasy-tavern-maker_rust_msa_wasm /dockerspace/
EXPOSE 9021
CMD ["fantasy-tavern-maker_rust_msa_wasm"]
