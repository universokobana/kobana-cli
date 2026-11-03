# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.2] - 2026-11-02

### Fixed

- **Mensagem dos erros do kadu** — erros no formato `{"error": {"code", "message"}}` mostravam "erro desconhecido"; agora mostram a mensagem que a API mandou

## [0.10.1] - 2026-11-02

### Changed

- **Página de retorno do login com a marca Kobana** — a aba que abre depois de autorizar o acesso no navegador agora segue o branding (fundo preto, verde-limão, Syne e Work Sans, a cegonha da marca ilustrada) no idioma do CLI. Os erros (autorização negada, retorno inválido) usam o mesmo layout, em roxo, com o código e a descrição do erro

## [0.10.0] - 2026-11-02

### Added

- **Login em qualquer produto** — `kobana <produto> auth login` (`kobana billing auth login`, `kobana kadu auth login`…) mostra os escopos que o produto publica no `/.well-known/oauth-protected-resource` do seu host MCP, lidos na hora (o `kadu`, que ainda não publica o documento, usa o catálogo da Conta Kobana filtrado por `kadu/`). Depois vêm os níveis de acesso, `read`, `write`, `delete`, `import` e `export`, que valem para todos os recursos escolhidos. O token sai da Conta Kobana com o `resource` do produto, então o `aud` é a API dele, e é renovado sozinho com o refresh token quando vence
- **Token por produto em variável de ambiente** — `KOBANA_BANKING_TOKEN`, `KOBANA_FINANCE_TOKEN`, `KOBANA_BILLING_TOKEN`, `KOBANA_INBOX_TOKEN`, `KOBANA_DUNNING_TOKEN`, `KOBANA_DASHBOARDS_TOKEN` e `KOBANA_KADU_TOKEN` (ou `KADU_TOKEN`), com prioridade sobre tudo
- **`auth status`, `auth export` e `auth logout` por produto** — `kobana <produto> auth status|export|logout`

### Changed

- **O login usa a Conta Kobana** (`account.kobana.com.br`, `account.sandbox.kobana.com.br`; a kobana-app local em development) no lugar de `app.kobana.com.br`; `KOBANA_ACCOUNT_URL` troca o endereço. Os escopos do `banking` passam a ser os publicados por ele (`banking/charge.bank_billets`…), e não mais a lista embutida no CLI
- **Credenciais salvas por produto e ambiente** (`credentials-<produto>-<ambiente>.enc`). O `credentials.enc` das versões anteriores continua valendo como credencial do `banking`
- **`auth` dentro do produto** — `kobana <produto> auth login|logout|status|export`, no formato `kobana <produto> <comando>` de todo o CLI. `--client-id`/`--client-secret` (client credentials) só valem para o `banking`

### Removed

- **`kobana auth …`** (incompatível) — use `kobana <produto> auth …`; o antigo `kobana auth login` equivale a `kobana banking auth login`
- **`KOBANA_TOKEN`** (incompatível) — o mesmo token ia para qualquer produto chamado. Use a variável do produto (`KOBANA_BANKING_TOKEN`, `KOBANA_FINANCE_TOKEN`…)

### Security

- **`rustls` 0.23.46** — fecha quatro alertas do Dependabot (2 moderados, 2 baixos) na pilha TLS por onde passam todas as requisições

## [0.9.0] - 2026-11-02

### Added

- **`kadu`: repositórios de código** — `kobana kadu sources list|create|get|update|delete|check`: repositórios git que uma tarefa lê numa cópia clonada a cada execução (`task.sources`). O `create` recebe `{"name", "repository": "owner/repo", "ref"}`; sem o GitHub App da organização no repositório, a resposta traz `deploy_key_public_key` para cadastrar como Deploy Key somente leitura, e `check` verifica o acesso e grava `access_status`. Spec e traduções do `kadu` atualizadas a partir do `main` do kadu, que traz também `usage performance` (`GET /v1/usage/performance`)
- **`kadu`: volumes** — `kobana kadu volumes list|create|get|update|delete`, mais `volumes files` (índice de arquivos do volume) e `volumes syncs` (o que cada execução montou e gravou). Spec e traduções do `kadu` renovadas a partir do `main` do kobana-agent-kadu
- **Arquivos em `--file campo=caminho`** — os endpoints que recebem `multipart/form-data` passam a funcionar pelo CLI: `kobana inbox emails create --file file=./mensagem.eml --json '{"inbox_id": "…"}'` injeta um `.eml` bruto, e o mesmo vale para `finance attachments create`, `billing certificates create`, `banking discharges create` e as importações do banking. Cada `--file` é uma parte do formulário, os campos de `--json` viram partes de texto, e o tipo vem da extensão (`.eml` → `message/rfc822`). Um endpoint que só recebe formulário manda o `--json` como formulário mesmo sem arquivo; `--file` num endpoint de JSON é recusado antes de enviar. O `--dry-run` mostra `content_type` e `files`
- **`kadu`: versões e consolidação de memórias** — `kobana kadu memories versions` (histórico, mais nova primeiro, `current: true` na vigente), `memories rollback` (`{"version": N}` grava uma versão nova com aquele conteúdo; o histórico fica inteiro), `memories consolidate create` (consolida as memórias das execuções terminadas de um período, com `task_id` opcional; `{"dry_run": true}` só conta) e `executions consolidate-memories` (uma execução). A memória traz `current_version` e `last_change`, e `usage get` ganhou `by_kind`, que separa execuções do agente, conversas e consolidação de memórias
- **`kadu`: servidores MCP da organização** — `kobana kadu mcp-servers list|create|get|update|delete`
- **`organization get|update` em `billing`, `dashboards`, `dunning`, `finance` e `inbox`** — a organização do token (`GET`/`PATCH /v1/organization`), no singular, sem id e sem listagem, como no `kadu`. `dunning`, `finance` e `inbox` ganham o recurso agora; em `billing` e `dashboards` ele existia como `organization list`, que segue como alias oculto. O `update` só muda o perfil que o tenant controla: nome, e-mail e telefone são espelho da Conta Kobana e saíram do corpo. Specs e traduções desses cinco produtos atualizadas a partir do `main` de cada app (o `inbox` deixou de exigir mTLS na spec)

### Changed

- **`dashboards`: ações do builder, do portal e das fontes de dados com nome de verbo** — `builder test-action|test-form|encrypt-secret|upload-og-image`, `portal-settings verify-sender` e um só `data-sources create` (com `kind: openapi|mcp|database`) no lugar de `builder action-runs|form-submissions|secrets|og-image`, `portal-settings sender-identity` e `data-sources openapi|mcp|database` (spec renovada). Os comandos antigos saem do `--help` mas continuam rodando, pelas rotas descontinuadas
- **Operação descontinuada sai do `--help` mas continua rodando** — um endpoint com `deprecated: true` na spec não aparece mais na lista de comandos (o substituto aparece), mas a linha antiga segue funcionando e o `kobana schema` informa `deprecated`. Primeiro caso: `billing usage credits daily`, substituído por `billing usage credits publish` (spec do `billing` renovada)
- **POST numa sub-rota vira ação do recurso** — um `POST` único num caminho abaixo de outra rota passa a ser um comando do recurso pai com o nome do segmento, como já acontecia depois de um id: `inbox inboxes recipients remove`, `billing holidays sync`, `billing plans bulk-archive`, `dunning tasks bulk-assign`, `dashboards workspaces import`, `dashboards builder publish`, `finance automatic-rules reorder`, `kadu memories consolidate`, `banking bank-billets cancel-all` e outros (53 comandos em todos os produtos). O nome antigo, com `create` no fim, continua funcionando como subcomando oculto (e em `kobana schema`), então scripts existentes não quebram
- **`kadu`: a organização é só a do token** (incompatível) — `kobana kadu organization get` e `organization update` (`GET`/`PATCH /v1/organization`), no singular e sem id, no lugar de `organizations list|get|update`. Não há mais listagem de organizações. As instalações do bot do Slack entram como `organization slack-installations list|update|delete`, só com o id da instalação. Spec e traduções do `kadu` atualizadas
- **GET de um registro só vira `get`** — uma rota sem id e sem paginação cuja resposta é `{ data: { … } }` (objeto, não lista) é um singular e agora se chama `get`: `me get` (`banking`, `inbox`, `dunning`, `kadu`), `kadu organization get`, `kadu repository get`, `kadu usage get`, `inbox usage cost get`/`usage summary get`, `finance cash-flow get` e os `aggregates get` do `finance`, `dunning metrics recovery get`. O nome antigo, `list`, continua respondendo como alias oculto (inclusive em `kobana schema`), então scripts existentes não quebram

### Removed

- **mTLS no `inbox`** — a API do Inbox deixou de usar mTLS em 31/10/2026, e o CLI não pede mais certificado de cliente: `KOBANA_INBOX_CLIENT_CERT` não é lida, e o aviso em stderr a cada chamada sem ela some. O suporte genérico (`client_cert_env`, `KobanaClient::with_client_cert`) continua para um produto que precise

### Fixed

- **Singular com envelope em `$ref`** — a inferência do `get` agora segue o `$ref` local da resposta (`#/components/schemas/OrganizationResponse`), como publicam `billing`, `dashboards` e `finance`: `me get`, `organization get`, `settings get`, `usage get` e os demais singulares desses produtos deixam de aparecer como `list` (alias oculto mantido). Um singular ao lado de uma rota de item (`dashboards data-purges` e `data-purges get <id>`) segue `list`, para não esconder o `get` por id

## [0.8.0] - 2026-11-01

### Added

- **Agentes e skills de IA no `finance`** — `kobana finance ai-agents` (com `triggers` e o `token` do gatilho) e `kobana finance ai-skills`
- **Produto `kadu` (Agente Kadu)** — `kobana kadu <recurso> <metodo>` fala com a API v1 do Kadu (`api.kadu.ia.br`): `tasks` (`run`, `readiness`, `endpoint`), `executions` (`cancel`, `continue`, `logs`, `events`, `todos`, `search`), `environments`, `secrets`, `schedules`, `files`, `memories`, `webhooks`, `webhook-deliveries`, `repository`, `usage` e o restante da API. Token RS256 da Conta Kobana com `aud=https://api.kadu.ia.br`; sem sandbox, e `--env development` vai para `localhost:3030/api`. Skill `kobana-kadu`

### Fixed

- **`--page-all` em rotas paginadas por cursor** — rotas que declaram o parâmetro `cursor` (`finance`, `inbox`, `kadu`) agora seguem `meta.next_cursor` até ele acabar. Antes o CLI mandava `page`/`per_page`, que essas APIs ignoram, e parava na primeira página sem avisar

### Changed

- **Specs de todos os produtos atualizadas** com o `main` de cada repositório, e traduções en/es refeitas. As APIs passaram a exigir um nível de acesso por operação — `read`, `write`, `delete`, `import`, `export` ou `all`, e um não vale por outro: `DELETE` exige `delete`, importação em volume exige `import` e exportação em volume exige `export`. Um token emitido antes precisa ser emitido de novo com esses níveis
- **Comandos que mudaram porque a rota mudou** (incompatível):
  - `billing`: exclusão em lote de `nfes`, `plans`, `products` e `proposals` é `bulk delete` (`DELETE /v1/<recurso>/bulk`), no lugar de `bulk-delete`; desarquivar grupo de produtos é `product-groups unarchive`; cancelar item de mudança de assinatura é `subscription-item-changes cancel`; arquivar plano ganhou `plans archive`
  - `dunning`: exclusão em lote de `people` e `charges` é `bulk delete` (`DELETE /v1/<recurso>/bulk`), no lugar de `bulk-archive create`; encerrar campanha é `negotiation-campaigns end`; `negativations`, `protests` e `tasks` não têm mais `delete` — use `remove`, `cancel` e `update` com `status: cancelled`
  - `dashboards`: o expurgo é `data-purges delete` (`DELETE /v1/data_purges`), no lugar de `data-purges create`
  - `inbox`: remover destinatário é `inboxes recipients remove create` (`POST /v1/inboxes/{id}/recipients/remove`), no lugar de `recipients delete`
  - `finance`: desvincular anexo é `attachments unlink` (`POST /v1/attachments/{id}/unlink`), no lugar do `DELETE` em `attachments links`
- **Rotas ainda em promoção**: no lançamento, `dashboards data-purges delete` e as exclusões em lote do `billing` (`bulk delete`) respondem `405` em produção até esses apps serem promovidos com as rotas novas

## [0.7.0] - 2026-10-28

### Added

- **Workspace no `dunning`** — a Régua de Cobrança passou a ter workspaces: `kobana dunning workspaces list` lista os que o token alcança, e todo comando de dados aceita o cabeçalho `X-Workspace-Id`, preenchido a partir de `KOBANA_WORKSPACE_ID` quando `--params` não o traz (como no `finance` e no `dashboards`). Opcional enquanto a organização tem um workspace só; com vários, a API responde `400 WORKSPACE_REQUIRED` sem ele

### Changed

- **Specs da Régua de Cobrança, do Faturamento Automático e do Dashboard Mágico atualizadas** com o `main` de cada app — a do dunning ganha o workspace; as outras duas, só textos revisados. Traduções en/es refeitas para todos os produtos

### Fixed

- **`--env sandbox` em `inbox` e `dashboards`** — os dois produtos não têm sandbox, e o CLI mandava as requisições para hosts que não existem. Agora termina com erro pedindo `--env production`, em vez de cair em produção sem avisar
- **Specs do Inbox e do Dashboard Mágico** não listam mais o servidor de sandbox

## [0.6.1] - 2026-10-27

### Changed

- **Specs do Gateway Bancário atualizadas** com o `kobana-api-specs` atual — mesmas operações, textos revisados
- **Traduções do Gateway Bancário completas** — as descrições em inglês e espanhol agora vêm dos arquivos `.en.json`/`.es.json` que o `kobana-api-specs` mantém ao lado de cada spec (os mesmos de que as docs são geradas), e cobrem todos os endpoints da v1 e da v2. Antes vinham de um snapshot das docs que deixava em português os endpoints novos ou renomeados (`financial companies`, `mailbox entries`, `data people` e outros)

## [0.6.0] - 2026-10-27

### Added

- **Script de instalação** — `curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash` baixa o binário da plataforma para `~/.kobana/bin`, confere o SHA-256 publicado na release e cria o link `~/.local/bin/kobana`, com banner e saída no estilo do instalador do Slack CLI. Fala português, inglês ou espanhol conforme o idioma do sistema (ou `KOBANA_LANG`). Aceita uma versão (`| bash -s -- v0.5.1`), `KOBANA_INSTALL_DIR`, `KOBANA_BIN_DIR` e `NO_COLOR`

- **CLI em português, inglês e espanhol** — ajuda, mensagens, erros, perguntas do `kobana auth login` e a página de retorno do login seguem o idioma do sistema (`LC_ALL`, `LC_MESSAGES`, `LANG`, depois o do sistema operacional), ou o de `KOBANA_LANG` ou `--lang pt|en|es`. As descrições dos endpoints no `--help` e no `kobana schema` vêm traduzidas das specs que cada produto publica nas suas docs (`specs/i18n/`, geradas por `scripts/build-spec-translations.rb`). Inclui os textos do próprio clap (`Uso:`, `Opções:`, erros de argumento) e um subcomando `kobana help <comando>` próprio
- **`Accept-Language`** em toda requisição, para as APIs que traduzem as próprias mensagens responderem no mesmo idioma

### Fixed

- **`kobana update` por um link simbólico** — o binário instalado pelo script é chamado por `~/.local/bin/kobana`; o update substituía o link em vez do binário. Agora resolve o caminho real antes

## [0.5.1] - 2026-10-26

### Fixed

- **Coleções sob um recurso pai viravam `get`** — um `GET` com parâmetro de caminho era sempre `get`, então `GET /webhooks/{id}/deliveries` (a lista) e `GET /webhooks/{id}/deliveries/{delivery_id}` (uma entrega) disputavam o mesmo nome e um dos dois sumia. Agora um `GET` que não termina em parâmetro e aceita paginação (`page`, `per_page`, `cursor`, `limit`) é `list`; recursos singulares sob um pai (`companies fiscal-profile`, `balances latest`) continuam `get`
  - **BREAKING:** listagens que se chamavam `get` passam a `list` — `banking financial accounts balances`, `statement-transactions`, `statement-transactions imports` e `statement-transactions syncs`; `billing customers statement` e `customers users`; `dashboards entities records`, `entities tenant-syncs`, `portal-users activity timeline` e `webhooks deliveries`; `dunning people contact-points` e `webhook-endpoints deliveries`; `finance financial-accounts balances`
  - Voltaram comandos que estavam inacessíveis: `banking financial accounts balances get`, `statement-transactions imports get` e `statement-transactions syncs get` (agora leem **um** item); `billing customers users get` e `billing nfes requests list`; `dashboards data-sources logs list`, `entities logs list` e `entities records get`; `inbox webhooks deliveries list`
- **Exemplos de `payables aggregates`** no README e na skill do `finance` — o comando é `payables aggregates list`

### Changed

- Specs do `dunning` e do `dashboards` atualizadas: a descrição da API do dunning fala no access token da Conta Kobana, e os resumos ambíguos do dashboards foram corrigidos

## [0.5.0] - 2026-10-26

Seis produtos no mesmo binário: Gateway Bancário (`banking`), Inbox Autônomo (`inbox`), Faturamento Automático (`billing`), Financeiro Inteligente (`finance`), Régua de Cobrança (`dunning`) e Dashboard Mágico (`dashboards`).

### Added

- **Produto `dunning` (Régua de Cobrança)** — `kobana dunning <recurso> <metodo>`, com clientes, cobranças, réguas, templates, notificações, acordos, disputas, negativações, protestos, tarefas, importações/exportações e webhooks. Spec OpenAPI v1 embutida (67 paths)
- **Produto `dashboards` (Dashboard Mágico)** — `kobana dashboards <recurso> <metodo>`, com fontes de dados, entidades, workspaces, portal do cliente, builder de páginas, backups e webhooks. Spec OpenAPI v1 embutida (153 paths)
- Skills `kobana-dunning` e `kobana-dashboards`
- **Parâmetros de cabeçalho** — cabeçalhos declarados na spec (como `X-Workspace-Id`) são passados em `--params` e enviados como cabeçalho, fora da query string. `KOBANA_WORKSPACE_ID` preenche o `X-Workspace-Id` do `finance` (obrigatório) e do `dashboards`. O `--dry-run` passa a mostrar os cabeçalhos e `kobana schema` os lista em `header_params`
- **Produto `billing` (Faturamento Automático)** — `kobana billing <recurso> <metodo>`, com assinaturas, planos, produtos, faturas, NF-e, propostas, cupons, pagamentos e contas de faturamento. Spec OpenAPI v1 embutida (173 paths)
- **Produto `finance` (Financeiro Inteligente)** — `kobana finance <recurso> <metodo>`, com contas financeiras, lançamentos, contas a pagar e receber, fluxo de caixa, conciliação e classificação. Spec OpenAPI v1 embutida (45 paths)
- Skills `kobana-billing` e `kobana-finance`
- **Produto `inbox` (Inbox Autônomo)** — `kobana inbox <recurso> <metodo>`, com workspaces, inboxes, e-mails, agentes, agent runs, webhooks (incluindo entregas e replay) e system events. Spec OpenAPI v1 embutida
- **Suporte a mTLS** — `KobanaClient::with_client_cert` apresenta um certificado de cliente no handshake TLS. A API do Inbox fica atrás de mTLS terminado no edge; aponte `KOBANA_INBOX_CLIENT_CERT` para um PEM com a cadeia e a chave privada. Sem ele o CLI avisa em stderr e segue (o dry-run não avisa, já que não faz requisição)
- Skill `kobana-inbox` documentando os recursos e as credenciais próprias do produto

### Changed

- **Specs atualizadas para as versões atuais das APIs** — `banking` v1 (67 paths) e v2 (158; novos `admin certificates`, `admin commands`, `data people`, `financial companies`, `financial statement-categories`, sincronização e resumo de extrato), `billing` (190; `late-fee-policies`, `bank-accounts`, `pix-keys`, `holidays`, `banks`, `credit-purchases`, `usage credits`, `me`), `finance` (55; `custom-classifiers`, `fiscal-documents`, `financial-accounts balances`, `me`) e `inbox` (31; `workflow-runs`, `secrets`, `models`, `usage`, `agents versions`, `me`). Os produtos fora do `banking` agora autenticam com token RS256 emitido pela Conta Kobana (`account.kobana.com.br`)
- **BREAKING (`finance`):** toda rota exige o cabeçalho `X-Workspace-Id` — defina `KOBANA_WORKSPACE_ID`
- **BREAKING (`banking`):** parâmetros de path renomeados na v2 — `entry_id` → `entry_uid` em `mailbox entries`, `id` → `uid` em `financial accounts` e nos `commands` de Pix e contas. `entry_uid` ainda aceita `uid`/`id` pelo alias; nos demais, passe `uid` em vez de `id`
- **Removido (`billing`):** `proposal-templates set-default`, que saiu da API
- **BREAKING: comandos agora exigem o produto** — a sintaxe mudou de `kobana <servico> <recurso> <metodo>` para `kobana <produto> <servico> <recurso> <metodo>`. A API original da Kobana passou a ser o produto Gateway Bancário (`banking`), então `kobana charge pix list` vira `kobana banking charge pix list` e `kobana v1 bank-billets list` vira `kobana banking v1 bank-billets list`
- **BREAKING: a versão da API saiu do comando** — `kobana banking v1 bank-billets list` vira `kobana banking bank-billets list` e `kobana inbox v1 emails list` vira `kobana inbox emails list`. A versão é resolvida a partir da spec e continua na URL da requisição; os domínios do v2 (`charge`, `payment`, …) seguem inalterados, porque são domínios e não versões. As specs de um produto passam a ser mescladas em uma árvore só
- **BREAKING: `kobana schema <endpoint>`** exige o produto e não aceita mais a versão: `kobana schema banking.charge.pix.create`, `kobana schema banking.bank-billets.list`. Caminhos com versão recebem um erro explicando a mudança e sugerindo o caminho novo. A saída de `kobana schema --list` passa a ser agrupada por produto

### Fixed

- **Descrições de recursos vazando entre produtos** — o texto de ajuda era mapeado só pelo nome do recurso, então `kobana billing payments` exibia "Pagamentos (unificado)" e `kobana finance transfers` exibia "Transferências (unificado)", ambos do Gateway Bancário. Agora o mapa é chaveado por `(produto, recurso)`
- **Panic na inicialização em paths de ação com mais de um verbo HTTP** — `/v2/admin/connections/{uid}/associations` (POST+DELETE), os canais de `/v2/mailbox/entries/{entry_id}/*` e `/v1/inboxes/{id}/recipients` geravam o mesmo nome de subcomando duas vezes, e o clap aborta nesse caso. Isso quebrava `kobana completions <shell>` e o `--help` desses recursos. Agora a ação vira um nó de recurso e cada verbo recebe seu método (`inboxes recipients create` / `... delete`)
- **User-Agent** — as requisições se identificavam como `kobana-cli/0.1.0` em qualquer versão; agora enviam a versão real

### Internal

- **Registro de produtos orientado a dados** (`crates/kobana-cli/src/product.rs`) — hosts, specs embutidas e layout de serviços de cada produto passam a ser uma entrada em `REGISTRY`. Adicionar um produto agora é adicionar uma spec em `specs/` e uma entrada na tabela; nenhum outro módulo do CLI conhece produtos
- **URL base por produto** — `Environment::base_url()` (que fixava `api.kobana.com.br`) foi removido; o host vem do produto resolvido, já que cada produto tem API própria
- **Prefixo de versão configurável** (`crates/kobana/src/spec.rs`) — `build_command_tree` recebe o prefixo a remover em vez de ter `/v1/`/`/v2/` hardcoded, que era uma amarra do banking dentro da biblioteca
- **`kobana schema`** passou a percorrer a mesma árvore de comandos usada no dispatch, em vez de reimplementar casamento de paths e inferência de método — elimina a duplicação que podia divergir do CLI real. A saída ganhou os campos `product` e `path_params`
- Specs renomeadas para `specs/banking-v1.json` e `specs/banking-v2.json`

## [0.4.1] - 2026-04-11

### Changed

- **`kobana update` output** — defaults to a single human-readable line; when an update is available and stdin is a terminal, prompts `(Y/n)` before running the upgrade command inline (`brew upgrade`, `cargo install`, or the standalone self-update)
- **`--json` flag** added to `kobana update` to restore the machine-readable JSON output (never prompts, safe in pipes)

## [0.4.0] - 2026-04-11

### Added

- **`kobana update` command** — checks GitHub Releases for a newer version and prints status (current, latest, install method, install command). With `--check`, only prints; without, performs an atomic self-update for standalone Unix binaries or prints the correct upgrade command for Homebrew/Cargo installs
- **Daily auto-update check** — every normal command performs a best-effort (3s timeout) check at most once per day and prints a yellow stderr warning when a newer release exists. Respects TTY (no ANSI codes when piped/logged)

### Changed

- **BREAKING: environment flags** — removed `--sandbox`, `--production`, `--development`. Use a single `--env <production|sandbox|development>` flag. The default is now **production** (was sandbox). `KOBANA_ENVIRONMENT` continues to work and accepts the same values
- **BREAKING: default environment** — now `production` instead of `sandbox`

## [0.3.0] - 2026-04-11

### Added

- **Interactive OAuth scope selector** — TUI drill-down for `auth login` when `--scopes` is not provided: first pick top-level groups, then sub-scopes per group, then read/write permission level
- **Human-readable scope descriptions** — pt-BR descriptions embedded for all 58 Kobana OAuth scopes and for top-level groups
- **Friendly OAuth denial page** — when the user declines on the consent screen, the browser shows a styled error page with the decoded reason instead of a connection-closed error; the CLI returns a descriptive auth error
- **UTF-8 URL decoding** on the OAuth callback query string

### Fixed

- **Credential key persistence** — enable `keyring` backend features (`apple-native`, `windows-native`, `sync-secret-service`); without features, keyring v3 silently "succeeded" and the encryption key was lost between runs, breaking every follow-up command after `auth login`
- **`resolve_token` error visibility** — propagate credential store errors instead of masking them behind a generic "No authentication configured" message

### Changed

- **`auth login` output** — richer JSON payload matching gws-cli style (credentials_file, encryption backend, scopes list)

## [0.2.2] - 2026-04-11

### Added

- **`--development` environment flag** — connects to `http://localhost:5005` (OAuth + API)
- **Full OAuth scopes list** — all 58 Kobana scopes requested by default
- **Verbose OAuth token exchange logging** — `--verbose` now shows request/response details for debugging
- **Custom `User-Agent`** on OAuth token requests (`kobana-cli/<version>`) to bypass CloudFront WAF

## [0.2.1] - 2026-04-10

### Added

- **Nix flake** — `nix run github:universokobana/kobana-cli` for installation and dev shell

### Fixed

- **OAuth token endpoint** — Use `app.kobana.com.br/oauth/token` instead of `api.kobana.com.br` (404)
- **OAuth redirect URI** — Use `127.0.0.1` instead of `localhost` to allow wildcard ports
- **OAuth default scope** — Request `read` only by default (was sending an invalid list of 57 resource scopes)

## [0.2.0] - 2026-04-09

### Added

- **PKCE authentication** — `kobana auth login` works zero-config with embedded public client_id `kobana-cli`
- **OAuth scopes** — All 57 Kobana scopes supported, customizable via `--scopes`
- **Agent skills** — SKILL.md files for all API modules (charge, payment, transfer, financial, admin, mailbox, data, security, v1)
- **Homebrew distribution** — `brew tap universokobana/tap && brew install kobana`
- **Project documentation** — README, AGENTS.md, CONTEXT.md, SECURITY.md, CHANGELOG, LICENSE, CLAUDE.md
- **Demo video** — VHS-generated GIF showcasing CLI features
- **CI/CD** — GitHub Actions with cross-platform builds (Linux/macOS/Windows) and automated releases

### Fixed

- **OAuth authorize URL** — Separate URLs per environment (sandbox: `app-sandbox.kobana.com.br`, production: `app.kobana.com.br`)
- **Clippy warnings** — Resolved `derivable_impls`, `manual_split_once`, `too_many_arguments`
- **GitHub Actions** — Upgraded to v5 to fix Node.js 20 deprecation warnings

## [0.1.0] - 2026-04-08

### Added

- **Core CLI** — Dynamic command tree generated from embedded OpenAPI specs (v1 + v2)
- **Two-phase parsing** — Identifies service from argv, builds clap commands from spec, re-parses
- **HTTP client** — reqwest-based with Bearer auth, idempotency keys, structured error handling
- **Authentication** — Token via `KOBANA_TOKEN`, OAuth2 Authorization Code and Client Credentials flows
- **Credential store** — AES-256-GCM encrypted credentials with OS keyring (macOS Keychain, etc.) and file fallback
- **Auth commands** — `kobana auth login`, `logout`, `status`, `export`
- **Schema introspection** — `kobana schema --list` and `kobana schema <endpoint>` for runtime API discovery
- **Output formats** — JSON (default), table, CSV via `--output-format`
- **Field masks** — `--fields` to limit response fields and protect AI agent context windows
- **Dry-run** — `--dry-run` for all mutations, shows request without executing
- **Auto-pagination** — `--page-all` with NDJSON streaming, `--page-limit`, `--page-delay`
- **Input validation** — Path traversal, URL injection, control character, and double-encoding detection
- **Structured exit codes** — 0 (success), 1 (API), 2 (auth), 3 (validation), 4 (schema), 5 (internal)
- **Structured logging** — stderr via `KOBANA_LOG`, JSON file rotation via `KOBANA_LOG_FILE`
- **Environment support** — `--sandbox` / `--production` flags, `KOBANA_ENVIRONMENT` env var
- **Config** — `.env` file loading, `KOBANA_CONFIG_DIR` for custom config directory
- **Helpers** — `+emitir` (boleto), `+cobrar` (Pix), `+cancelar-lote` (batch cancel)
- **Shell completions** — bash, zsh, fish, powershell, elvish via `kobana completions <shell>`
- **API coverage** — Full v1 (boletos, clientes, webhooks) and v2 (charge, payment, transfer, financial, admin, mailbox, data, EDI, security)
