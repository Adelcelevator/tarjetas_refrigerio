FROM docker.io/library/rust:latest AS COMPILA
ENV DIRECTORIO=tarjetas_refrigerio
WORKDIR /${DIRECTORIO}
COPY . .
RUN apt -y update && apt -y upgrade
RUN cargo build -r -j12
FROM docker.io/library/debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 glibc-source glibc-doc && rm -rf /var/lib/apt/lists/*
WORKDIR /home/tarjetas/
COPY --from=COMPILA /tarjetas_refrigerio/target/release/tarjetas_refrigerio ./
EXPOSE 8080
CMD [ "/home/tarjetas/tarjetas_refrigerio" ]
