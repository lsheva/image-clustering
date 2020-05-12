FROM node

RUN apt-get install \
  ca-certificates \
  gcc 

RUN curl https://sh.rustup.rs -sSf | \
  sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

RUN yarn global add neon-cli

WORKDIR /app

COPY . .

CMD neon build --release && node ./test/test3.js
