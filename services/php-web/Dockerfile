FROM php:8.3-fpm-alpine

# Системные зависимости
RUN apk add --no-cache \
    bash \
    curl \
    git \
    unzip \
    rsync \
    postgresql-dev

# PHP расширения
RUN docker-php-ext-install pdo pdo_pgsql

# Composer (без composer image)
RUN curl -sS https://getcomposer.org/installer | php -- \
    --install-dir=/usr/local/bin \
    --filename=composer

# Рабочая директория
WORKDIR /var/www/html

# entrypoint
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD ["php-fpm"]
