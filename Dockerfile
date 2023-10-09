#Simple Dockerfile to build togglempc as a docker image
FROM debian:bullseye as build-togglempc

RUN apt-get update && apt-get install -y --no-install-recommends build-essential git curl ca-certificates && apt-get clean
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN git clone https://github.com/christianwicke/togglempc.git
RUN cd togglempc/ && /root/.cargo/bin/cargo build --release

FROM debian:bullseye
COPY --from=build-togglempc /togglempc/target/release/togglempc .
COPY --from=build-togglempc /togglempc/sample-config.toml .
ENTRYPOINT ["/togglempc"]
CMD ["sample-config.toml"]
