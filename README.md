# Rust In Enterprise Workshop - Rust Nation 2024


## Run the CLI

```bash
# Working
echo "hello world" | cargo run --bin cli -- --max-token-length 10
# Errors
echo "hello looooooooooooooong password123" | cargo run --bin cli -- --max-token-length 10
```

## Run the Web Server

### Server

```
cargo run --bin web -- --max-token-length 10
```

### Client

I reccomend installing [jq](https://jqlang.github.io/jq/) and piping the results of `curl` into `jq` for better formatting of the result.

```bash
# Working
curl localhost:8080/audit -X POST -H "Content-Type: application/json" -d'{"text": "hello world"}' | jq
# Errors
curl localhost:8080/audit -X POST -H "Content-Type: application/json" -d'{"text": "hello looooooooooooooong password123"}' | jq
```

## Via Python

```bash
python -m venv venv
source venv/bin/activate
pip install maturin
maturin develop
python
```

```python
from auditor_lib import Auditor
auditor = Auditor(["password123"], 9)
# Working
auditor.audit("hello world")\
# Errors
auditor.audit("hello looooooooooooooong password123")
```
