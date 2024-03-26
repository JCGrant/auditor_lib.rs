import tomllib
from auditor_lib import Auditor

with open("config.toml", "rb") as f:
    config = tomllib.load(f)

auditor = Auditor(config["disallowed_strings"], config["max_token_length"])

# Working
auditor.audit("hello world")
print("SUCCESS")

# Errors
auditor.audit("hello looooooooooooooong password123")
print("SHOULD NOT SEE")
