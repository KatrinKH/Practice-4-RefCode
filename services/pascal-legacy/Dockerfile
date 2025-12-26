FROM debian:12-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    fp-compiler \
    fp-units-base \
    fp-units-fcl \
    ca-certificates \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/legacy
COPY legacy.pas ./
COPY run.sh /usr/local/bin/run_legacy.sh
RUN chmod +x /usr/local/bin/run_legacy.sh

CMD ["/usr/local/bin/run_legacy.sh"]
