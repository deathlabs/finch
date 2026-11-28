# Preview and Build the Docs

Install uv and Python 3.11 or later. From the repository root, start the Zensical preview server.

```bash
make docs
```

Open `http://127.0.0.1:8001`. Edit the pages under `docs/src/content` and the navigation in `docs/src/zensical.toml`.

If Windows cannot reach the WSL preview through localhost, run these commands in WSL to serve the docs on its network address.

```bash
wsl_ip=$(hostname -I | awk '{print $1}')
make docs DEV_ADDR="$wsl_ip:8002"
```

Open the URL that Zensical prints in the Windows browser.

To build the static site, run:

```bash
cd docs/src
uv run --locked zensical build --config-file zensical.toml
```

The build writes the site to `docs/src/public`.
