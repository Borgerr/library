# Library

A persistent library of randomized books, right from your browser.
Puts the philosophy of monkeys with typewriters to the test.
See if you can generate Shakespeare, and share that with your friends.

## Deploying

### Prerequisites

You should have the common build utilities for [rust](<rustup.rs>)
and virtualization utilities from [docker](<docker.com>).
Specifically, you need docker compose.

Once installed, docker compose is configured to deploy a PostgreSQL database
and a "library manager" webserver that communicates with it.

You will need a `.env` file in the root directory. Your `.env` file should look something like:

```sh
POSTGRES_USER=USER
POSTGRES_PASSWORD=thisisagoodpassword
POSTGRES_NAME=postgres
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@library-db:${DB_PORT}/${POSTGRES_NAME}
DB_PORT=5432
LIBMGR_ADDR=0.0.0.0
LIBMGR_PORT=3000
```

## Eventual Plans

These are in no particular order, but may be niceties for admins and users of the site.

- [ ] Controlling how much space the library may occupy
- [ ] Pre-population scripts (so users don't have to wait as long)
- [ ] Specifying a style.css
- [ ] Cmdline flags for specifying strict backend (i.e., modular frontend beyond `get_book`)
- [ ] (?) Customized, "dependency injected" pseudo-random algorithm? Very vague idea, may not be pursued at all.

