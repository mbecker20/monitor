FROM rust:latest as builder
WORKDIR /builder

COPY ./core ./core

COPY ./lib/types ./lib/types
COPY ./lib/helpers ./lib/helpers

COPY ./lib/db_client ./lib/db_client
COPY ./lib/periphery_client ./lib/periphery_client
COPY ./lib/axum_oauth2 ./lib/axum_oauth2

RUN cd core && cargo build --release

FROM gcr.io/distroless/cc

COPY ./frontend/build /frontend

COPY --from=builder /builder/core/target/release/core /

EXPOSE 9000

CMD ["./core"]