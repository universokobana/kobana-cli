---
id: languages
title: Idiomas
sidebar_label: Idiomas
sidebar_position: 7
description: El CLI habla portugués, inglés y español — ayuda, mensajes, errores y descripciones de los endpoints.
---

# Idiomas

Todo lo que `kobana` le dice a una persona sale en **portugués, inglés o español**: ayuda, mensajes de progreso, preguntas, errores y la página que muestra el navegador al final del inicio de sesión. Las descripciones de los endpoints (en `--help` y en `kobana schema`) vienen traducidas de las especificaciones que publica cada producto.

## Cómo se elige el idioma

En este orden:

1. `--lang pt|en|es`
2. `KOBANA_LANG`
3. `LC_ALL`, `LC_MESSAGES`, `LANG`
4. El idioma del sistema

Los locales `pt*` dan portugués, `es*` dan español y cualquier otro da inglés.

```bash
kobana --lang en banking bank-billets --help
export KOBANA_LANG=es
```

## Qué no cambia

Los nombres de comandos y flags (incluidos los helpers en portugués, como `+emitir --valor`), las claves del JSON y los códigos de salida son los mismos en cualquier idioma, por eso un script escrito en un idioma funciona en cualquier máquina.

El CLI también envía `Accept-Language`, y las APIs que traducen sus propios mensajes responden en el mismo idioma.
