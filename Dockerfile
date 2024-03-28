# Используем конкретную версию Rust для сборки, которая поддерживает редакцию 2021
FROM rust:1.76.0 AS builder

WORKDIR /app

# Копируем файлы для сборки
COPY . .

# Выполняем сборку в режиме выпуска
RUN cargo build --release

# Используем минимальный базовый образ для запуска
FROM debian:buster-slim

# Создаем каталог для приложения
WORKDIR /app

# Копируем исполняемый файл из сборки
COPY --from=builder /app/target/release/Jenkins .

# Задаем ENTRYPOINT для запуска приложения
ENTRYPOINT ["./Jenkins"]