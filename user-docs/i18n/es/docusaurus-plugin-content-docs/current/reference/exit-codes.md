---
id: exit-codes
title: Códigos de salida
sidebar_label: Códigos de salida
sidebar_position: 3
description: Qué significa cada código de salida de kobana, para manejar errores en scripts.
---

# Códigos de salida

| Código | Significado |
|---|---|
| `0` | Éxito |
| `1` | Error de API (respuesta 4xx o 5xx) |
| `2` | Error de autenticación |
| `3` | Error de validación, incluida una línea de comando inválida (comando o flag desconocidos, argumento faltante) |
| `4` | Error de schema |
| `5` | Error interno |

Los errores de API, de autenticación y de validación salen como un objeto JSON `{"error": {"code": …, "message": …}}`. Una línea de comando que no tiene sentido (comando o flag desconocidos) sale en texto, por el stderr, con el código `3`.

```bash
if ! kobana billing invoices get "$ID" > factura.json; then
  case $? in
    1) echo "La API rechazó la solicitud" ;;
    2) echo "Inicia sesión de nuevo: kobana billing auth login" ;;
    3) echo "Comando o argumento inválido" ;;
  esac
fi
```
