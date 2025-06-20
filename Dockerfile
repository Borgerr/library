FROM rust:1.87-bookworm

ARG LIBRARY_MGR_PATH=/opt/library-mgr

WORKDIR $LIBRARY_MGR_PATH

COPY . .

RUN cargo install --path .

EXPOSE ${LIBRARY_MGR_PORT}

CMD ["library"]

