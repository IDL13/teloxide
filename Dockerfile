# Используем конкретную версию Rust для сборки, которая поддерживает редакцию 2021
FROM rust:1.76.0

WORKDIR /app

# Копируем файлы для сборки
COPY . .

# Создаем каталог для приложения
WORKDIR /app

# Копируем исполняемый файл из сборки
COPY /target/release/Jenkins .

# Задаем ENTRYPOINT для запуска приложения
ENTRYPOINT ["./Jenkins"]