# Security Policy

## Reporting a Vulnerability

**Please do not open public GitHub issues for security vulnerabilities.**

If you discover a security vulnerability in this project, please report it through Kobana's White Hat program:

**https://www.kobana.com.br/en/white-hat**

When reporting, please include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will acknowledge receipt within 48 hours and work with you to understand and address the issue.

## Verifying releases

Release binaries carry a build provenance attestation (GitHub Artifact Attestations, signed with Sigstore; no long-lived signing key). To check that a binary was built by this repository's release workflow:

```bash
gh attestation verify kobana-linux-amd64 --repo universokobana/kobana-cli
```

The SHA-256 checked by `install.sh` and `kobana update` comes from the same place as the binary, so it only detects corrupted downloads.

---

# Política de Segurança

## Reportando uma Vulnerabilidade

**Por favor, não abra issues públicas no GitHub para vulnerabilidades de segurança.**

Se você descobrir uma vulnerabilidade de segurança neste projeto, reporte através do programa White Hat da Kobana:

**https://www.kobana.com.br/white-hat**

Ao reportar, inclua:

- Descrição da vulnerabilidade
- Passos para reproduzir
- Impacto potencial
- Sugestão de correção (se houver)

Confirmaremos o recebimento em até 48 horas e trabalharemos com você para entender e resolver o problema.

## Verificando releases

Os binários das releases têm um atestado de proveniência (GitHub Artifact Attestations, assinado via Sigstore; sem chave de longa duração). Para conferir que um binário foi gerado pelo workflow de release deste repositório:

```bash
gh attestation verify kobana-linux-amd64 --repo universokobana/kobana-cli
```

O SHA-256 conferido pelo `install.sh` e pelo `kobana update` vem do mesmo lugar do binário, então só detecta downloads corrompidos.
