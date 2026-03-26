# Especificação Criptográfica e Formatos

Este documento especifica formatos e entradas criptográficas do sistema `e2easy-pc`, com conformidade obrigatória à **RFC 8785** para operações de hash e assinatura.

As funções de hash para escalares e elementos seguem **RFC 9380: Hashing to Elliptic Curves**, implementando `hash_to_field` (para escalares) e `hash_to_curve` (para pontos) com segurança criptográfica e distribuição uniforme.

## RFC 8785: Canonicalização

Todas as operações de hash/assinatura **DEVEM** usar:

- Chaves em **ordem alfabética**
- **Sem espaços** (compacto)
- **Sem quebras de linha**

## Tipos (codificação)

- `TrackingCode`: string hexadecimal **maiúscula** (64 caracteres, resultado de SHA-256)
- `Scalar`: string hexadecimal **maiúscula** (64 caracteres, 32 bytes)
- `Element`: string hexadecimal **maiúscula** (66 caracteres, ponto comprimido em P-256)
- `timestamp`: RFC3339 (ex.: `"2026-03-05T02:09:25.467237740+00:00"`)
- `verifying_key`: DER hexadecimal maiúscula

## Transformação de voto em escalar

Esta transformação é usada para calcular os compromissos de Pedersen e o código de rastreio, e **não** altera a representação JSON canônica.

### `Vote → bytes (8 bytes)`

- Ordem dos campos conforme a implementação atual de `Vote::to_bytes()`:
  - 4 bytes big-endian de `contest`
  - 4 bytes big-endian de `choice`
- Forma: `contest || choice`

### `bytes → Scalar (32 bytes)`

- O vetor de 8 bytes é preenchido à esquerda com zeros até 32 bytes.
- Os 8 bytes do voto ficam nas posições finais (`[24..32]`).
- O escalar é criado de forma estrita: apenas aceita representação canônica de 32 bytes (sem redução modular). Isto é crucial para garantir bijetividade: cada Vote tem um Scalar único e vice-versa.
- Como 8 bytes (= $2^{64}$) ≪ ordem do grupo P-256 (≈ $2^{256}$), a operação sempre tem sucesso.

### `Scalar → Vote`

- Obtêm-se os 32 bytes do escalar.
- Extraem-se os últimos 8 bytes (`[24..32]`).
- Reconstrói-se o `Vote` lendo:
  - `contest`: primeiros 4 bytes desse bloco
  - `choice`: últimos 4 bytes desse bloco

### Exemplo

Para `choice = 3` e `contest = 1`:

- `Vote::to_bytes()` = `00000001 00000003`
- _Padding_ para 32 bytes = `0000000000000000000000000000000000000000000000000000000100000003`

## Derivação determinística de nonces

Usada para gerar nonces reprodutíveis a partir de uma `seed` do tipo `Scalar`.

- Entrada: `seed: Scalar`, `count: usize`
- Saída: `Vec<Scalar>` com `count` elementos
- Algoritmo: para cada `i` em `0..count`, `nonce_i = hash2scalar((seed, i))`
- Caso limite: se `count = 0`, retorna vetor vazio

### Serialização para hash (nonces)

- A tupla `(seed, i)` é serializada em JSON canônico (RFC 8785) antes do SHA-256.
- `seed` é serializado como string hexadecimal maiúscula (32 bytes = 64 caracteres).
- `i` é serializado como número inteiro JSON.

### Exemplo de entrada canônica (nonces)

```json
["A9C8563BF45F1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234",0]
```

### Função de hash (nonces)

- `hash2scalar(obj)` implementa `hash_to_field` (RFC 9380, Seção 5.2) usando `expand_message_xmd` com SHA-256 (RFC 9380, Seção 5.3.1).
- O resultado é sempre um escalar válido no campo do grupo P-256, com distribuição uniforme mesmo para entradas pequenas.
- DST (Domain Separation Tag): `"E2EASY-PC/NONCE/V1"`

### Propriedades de segurança

- Determinismo: mesma `seed` e mesmo `count` geram exatamente a mesma sequência.
- Separação por índice: para uma `seed` fixa, cada `i` produz um nonce único associado a essa posição.
- Separação de domínio: a DST garante que nonces não se confundem com valores derivados em outros contextos.
- Reprodutibilidade para auditoria: terceiros podem recomputar os nonces se a `seed` for conhecida.
- Recomendação operacional: não reutilizar a mesma `seed` em contextos criptográficos distintos.

## Derivação determinística de h_list

Usada para gerar a lista de elementos de base para _commitments_ Pedersen a partir de uma `seed` do tipo string.

- Entrada: `seed: &str`, `count: usize`
- Saída: `Vec<Element>` com `count` elementos
- Algoritmo: para cada `i` em `0..count`, `h_list[i] = hash2element((seed, i))`
- Caso limite: se `count = 0`, retorna vetor vazio

### Serialização para hash (h_list)

- A tupla `(seed, i)` é serializada em JSON canônico (RFC 8785) antes do hash.
- `seed` é serializado como string JSON (entre aspas).
- `i` é serializado como número inteiro JSON.

### Exemplo de entrada canônica (h_list)

```json
["3.141592653589793238462643383279502",0]
```

### Função de hash (h_list)

- `hash2element(obj)` implementa `hash_to_curve` (RFC 9380, Seção 3) usando `expand_message_xmd` com SHA-256 (RFC 9380, Seção 5.3.1) para mapeamento determinístico.
- Qualquer entrada produz um ponto válido na curva P-256, sem necessidade de verificação.
- DST (Domain Separation Tag): `"E2EASY-PC/HLIST/V1"`

### Propriedades para h_list

- Determinismo: mesma `seed` e mesmo `count` geram exatamente a mesma sequência.
- Separação por índice: para uma `seed` fixa, cada `i` produz um elemento único associado a essa posição.
- Separação de domínio: a DST garante que elementos de h_list não se confundem com valores derivados em outros contextos.
- Reprodutibilidade para auditoria: terceiros podem recomputar h_list se a `seed` for conhecida.
- Armazenamento: apenas a `seed` é armazenada em `election_config.json`, e não a lista completa de elementos (otimização de espaço).

## Arquivos emitidos

### `config/election_config.json`

```json
{"contests":[{"contest_id":0,"name":"contest_0","options":[{"name":"opcao_0","option_id":0},{"name":"opcao_1","option_id":1}]}],"crypto":{"h":"026FA250...","h_list_seed":"3.141592653589793238462643383279502"}}
```

### `outputs/rdv_prime.json`

```json
{"entries":[{"choice":3,"contest":0},{"choice":1,"contest":1}]}
```

### `outputs/rdcv.json`

```json
{"entries":[{"committed_votes":["02B90AFF..."],"timestamp":"2026-03-05T01:59:39...","tracking_code":"C3FF3E7B..."}],"head":"D7442C69...","tail":"A9C8563B..."}
```

### `outputs/rdcv_prime.json`

```json
{"entries":["037F3F53...","03AAA787..."]}
```

### `outputs/zkp_output.json`

```json
{"m_list":["00000000..."],"r_list":["4E3C9357..."],"shuffle_proof":{...},"verifying_key":"3059301306..."}
```

### `outputs/*.sig`

```json
"33F94C55F91935E1662299012E0AC8891C9907BDA7F12D9C7FAFA46CE373FCD2800961A333184FDA090704DE9C0094A9E9BDD6D1663556AC1C0BEF127E7C901C"
```

## Entradas de hash

### Código de rastreio (`E2Easy::vote`)

```json
["A9C8563BF45F...","2026-03-05T02:09:25.467237740+00:00",["03EFCDAB4451...","0282BAF46ED1..."]]
```

Tupla com 3 elementos: `(prev_tracking_code, timestamp, committed_votes)`.

### Fechamento (`E2Easy::tally`)

```json
["A9C8563BF45F...","CLOSE"]
```

Tupla com 2 elementos: `(prev_tracking_code, "CLOSE")`.
