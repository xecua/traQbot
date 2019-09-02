FROM rustlang/rust:nightly

ENV DOCKERIZE_VERSION v0.6.1

RUN cargo install diesel_cli --no-default-features --features mysql && \
    curl -sL https://deb.nodesource.com/setup_6.x | bash && \
    apt install -y wget && \
    wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    apt purge -y --auto-remove wget && \
    apt clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

