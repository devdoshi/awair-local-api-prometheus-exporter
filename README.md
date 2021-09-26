# Getting Started
If you have enabled the Awair Local API on your compatible Awair device, note the URL you use to view the latest air data. It should look like: `http://1.2.3.4/air-data/latest`. You'll need to provide that as the value for the `AWAIR_LOCAL_URL` environment variable when you run this code.

## Locally
```bash
AWAIR_LOCAL_URL=http://1.2.3.4/air-data/latest cargo run
```

## Docker
```bash
docker build --rm . -t awair-local
docker run -e AWAIR_LOCAL_URL=http://1.2.3.4/air-data/latest -p 9185:9185 awair-local
```