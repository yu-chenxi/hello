# typical rust build file
# FROM rust AS builder
# WORKDIR hello
# COPY . .
# RUN cargo build --release

# FROM ubuntu
# COPY --from=builder /hello/target/release/hello /bin
# CMD ["/bin/hello"]

FROM ubuntu
COPY . .
RUN ls
CMD ["/bin/hello"]