# Etapa de compilación
FROM rust:1.75-bookworm as builder

WORKDIR /usr/src/app

# Copiar manifiestos de Cargo
COPY Cargo.toml Cargo.lock ./

# Crear un proyecto vacío con las dependencias para aprovechar el caché de Docker
RUN mkdir -p src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/sai*

# Copiar el código fuente real
COPY . .

# Compilar para producción
RUN cargo build --release

# Etapa de producción
FROM debian:bookworm-slim

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Crear un usuario no privilegiado para ejecutar la aplicación
RUN groupadd -r saiuser && useradd -r -g saiuser saiuser

WORKDIR /app

# Copiar el binario compilado desde la etapa de compilación
COPY --from=builder /usr/src/app/target/release/sai /app/
COPY --from=builder /usr/src/app/configs /app/configs

# Copiar archivos de configuración
COPY --from=builder /usr/src/app/.env.example /app/.env

# Establecer permisos adecuados
RUN chown -R saiuser:saiuser /app
USER saiuser

# Exponer el puerto que utilizará la aplicación
EXPOSE 8080

# Comando para ejecutar la aplicación
CMD ["./sai"]

