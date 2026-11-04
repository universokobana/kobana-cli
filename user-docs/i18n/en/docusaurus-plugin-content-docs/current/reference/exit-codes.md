---
id: exit-codes
title: Exit codes
sidebar_label: Exit codes
sidebar_position: 3
description: What each kobana exit code means, so you can handle errors in scripts.
---

# Exit codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | API error (4xx or 5xx response) |
| `2` | Authentication error |
| `3` | Validation error, including an invalid command line (unknown command or flag, missing argument) |
| `4` | Schema error |
| `5` | Internal error |

API, authentication and validation errors come out as a JSON object `{"error": {"code": …, "message": …}}`. A command line that makes no sense (unknown command or flag) comes out as text, on stderr, with code `3`.

```bash
if ! kobana billing invoices get "$ID" > fatura.json; then
  case $? in
    1) echo "The API rejected the request" ;;
    2) echo "Sign in again: kobana billing auth login" ;;
    3) echo "Invalid command or argument" ;;
  esac
fi
```
