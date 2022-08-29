FROM paritytech/ci-linux:production

RUN mkdir -p /tmp/dev

WORKDIR /tmp/dev

COPY . .

EXPOSE 9944:9944