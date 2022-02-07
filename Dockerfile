# Version form Node 17
FROM node:17

WORKDIR /app

# Copies package json & installs JS dependicies
COPY client/package.json client/package.json
RUN npm install --prefix ./client

# Installs Rust Toolchain
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Install web-sys
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

#  Copy rust source and js source
COPY client client
COPY lib lib

# Exposes port
EXPOSE 3000

# Builds app
CMD [ "npm", "run", "build", "--prefix", "./client"]
