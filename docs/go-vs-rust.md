# Go vs Rust — Analise para o Kobana CLI

## Resumo

| Criterio | Go | Rust |
|----------|------|------|
| Tempo de compilacao | Rapido (segundos) | Lento (minutos, especialmente com muitas deps) |
| Tamanho do binario | ~10-15 MB (estatico) | ~5-10 MB (com LTO) |
| Cross-compilation | Trivial (`GOOS=linux GOARCH=amd64`) | Requer toolchains por target |
| Curva de aprendizado | Baixa | Alta (ownership, lifetimes, borrow checker) |
| Ecossistema CLI | Excelente (cobra, viper, bubbletea) | Excelente (clap, serde, ratatui) |
| Ecossistema HTTP | Excelente (net/http nativo, resty) | Excelente (reqwest, hyper) |
| JSON parsing | Nativo (`encoding/json`) | Excelente (serde_json, muito rapido) |
| Async/concurrency | Goroutines (nativo, trivial) | Tokio runtime (mais complexo) |
| Seguranca de memoria | GC, sem null safety forte | Garantida em compile-time, sem GC |
| Performance | Muito boa | Superior (sem GC, zero-cost abstractions) |
| Binario estatico | Default | Default com musl |
| Distribuicao npm | Facil (postinstall download) | Igual |
| Community/hiring | Ampla, muito popular no Brasil | Menor, crescendo |

---

## Pros do Go

### 1. Velocidade de desenvolvimento
Go e deliberadamente simples. Menos decisoes de design, menos tempo debugando lifetimes. Para um CLI que e essencialmente "parse args -> HTTP request -> print JSON", a simplicidade do Go e uma vantagem real.

### 2. Cross-compilation trivial
```bash
GOOS=linux GOARCH=amd64 go build -o kobana-linux-amd64
GOOS=darwin GOARCH=arm64 go build -o kobana-darwin-arm64
GOOS=windows GOARCH=amd64 go build -o kobana-windows-amd64.exe
```
Sem instalar toolchains extras. Fundamental para distribuir binarios para multiplas plataformas.

### 3. Concorrencia nativa
Goroutines tornam trivial fazer requests paralelos (ex: paginar multiplos recursos ao mesmo tempo). Em Rust, precisa configurar Tokio runtime e lidar com `async`/`await` + `Send`/`Sync` bounds.

### 4. Ecossistema CLI maduro
- **cobra** — framework de CLI mais usado, com subcommands, completions, man pages
- **viper** — configuracao (env vars, config files, flags)
- **bubbletea/lipgloss** — TUI interativa (para wizards de auth)
- **survey/huh** — prompts interativos

### 5. Onboarding de contribuidores
Go tem curva de aprendizado muito menor. Qualquer dev backend consegue contribuir rapidamente. Rust exige dominar ownership, lifetimes, traits, e o borrow checker.

### 6. Tempo de compilacao
Em Go, o ciclo edit-compile-test e quase instantaneo. Em Rust, compilacoes incrementais levam 5-15s e full builds podem levar minutos.

---

## Pros do Rust

### 1. Seguranca de memoria garantida
O borrow checker elimina classes inteiras de bugs em compile-time: use-after-free, data races, null pointer dereferences. Para um CLI que lida com credenciais e dados financeiros, isso e relevante.

### 2. Performance superior
Sem garbage collector, zero-cost abstractions. Para operacoes de paginacao em massa (--page-all com milhares de paginas), Rust sera mais eficiente em memoria e CPU.

### 3. Serde e o melhor parser JSON do ecossistema
`serde` + `serde_json` sao extremamente rapidos e ergonomicos. Deserializacao tipada e trivial com `#[derive(Deserialize)]`. Em Go, `encoding/json` e mais lento e structs com `json:"field"` tags sao mais verbosas.

### 4. Clap e extremamente poderoso
```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    service: Service,
    #[arg(long)]
    dry_run: bool,
}
```
Derive macros geram CLI completa com validacao, completions e man pages. Comparavel ao cobra do Go, mas com type-safety em compile-time.

### 5. Binario menor
Com LTO (Link-Time Optimization), binarios Rust sao tipicamente menores que equivalentes Go. Relevante para distribuicao.

### 6. Consistencia com gws-cli
O gws-cli (referencia de design) e escrito em Rust. Reutilizar patterns, crates e ate codigo seria possivel. Menor gap cognitivo ao consultar a implementacao de referencia.

### 7. Enums para modelar estados
O type system do Rust (enums com data, pattern matching exaustivo) e ideal para modelar estados de boletos, pagamentos e transferencias sem bugs de estado invalido.

---

## Contras do Go

- **Tratamento de erros verboso** — `if err != nil` em cada chamada. Sem `?` operator como Rust.
- **Sem generics robustos** — Generics existem desde Go 1.18 mas sao limitados comparados a traits/generics de Rust.
- **Null safety fraca** — Ponteiros podem ser nil. Structs tem zero values que podem mascarar bugs.
- **JSON parsing mais lento** — `encoding/json` usa reflection. Para payloads grandes (page-all), pode ser gargalo.
- **Dependency management** — Go modules funciona, mas o ecossistema de vendoring/versioning e menos sofisticado que Cargo.

## Contras do Rust

- **Curva de aprendizado** — Lifetimes, ownership e trait bounds podem travar desenvolvedores menos experientes.
- **Tempo de compilacao** — Full build de um CLI com tokio + reqwest + clap + serde leva 1-3 minutos.
- **Cross-compilation** — Precisa de toolchains (ex: `x86_64-unknown-linux-musl`). Resolvido com `cross` ou CI matrix, mas e mais trabalho que Go.
- **Async complexity** — `async`/`await` em Rust e mais complexo. Pinning, `Send` bounds, e escolha de runtime (tokio) adicionam complexidade.
- **Menos devs disponiveis** — Rust e menos popular no Brasil. Encontrar contribuidores pode ser mais dificil.

---

## Recomendacao

### Se a prioridade e velocidade de entrega e facilidade de manutencao: **Go**

O kobana CLI e fundamentalmente um "thin client" — parsing de argumentos, chamadas HTTP, formatacao de JSON. Go faz isso com menos codigo, compilacao mais rapida e facilidade de cross-compilation. A equipe pode iterar rapidamente e aceitar contribuicoes externas com menor barreira.

### Se a prioridade e robustez, performance e alinhamento com gws-cli: **Rust**

Se voce planeja reutilizar patterns do gws-cli, fazer parsing dinamico de OpenAPI specs em runtime, ou precisa de garantias fortes de seguranca de memoria para lidar com credenciais financeiras, Rust e a escolha mais solida.

---

## Decisao: Rust

O desenvolvimento sera feito pelo Claude Code, o que elimina a principal desvantagem do Rust (curva de aprendizado) e amplifica suas vantagens:

1. **Claude Code domina o ecossistema Rust** — clap, serde, reqwest, tokio
2. **gws-cli como referencia direta** — mesmo repo, mesma linguagem, patterns reutilizaveis
3. **Type safety para operacoes financeiras** — borrow checker previne classes inteiras de bugs em compile-time
4. **Curva de aprendizado irrelevante** — nao ha dev humano aprendendo lifetimes
5. **Tempo de compilacao irrelevante** — Claude Code nao itera em trial-and-error

Linguagem definida: **Rust**. Toolchain: clap + serde + reqwest + tokio.
