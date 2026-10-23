# Especificação Técnica: Otimização de Binário e Linking Estático musl

- **Ticket**: #1
- **Escopo**: Configuração de LTO fat e strip em `[profile.release]` e flags de linking estático com musl libc no simulador `kokorosim`.

## 1. Contexto e Motivação
A configuração atual utilizava `lto = true` e omitia `strip = true`. Além disso, para garantir compatibilidade com distribuições Linux sem dependência do interpretador dinâmico de libc musl, `.cargo/config.toml` deve estipular `-C target-feature=+crt-static`.

## 2. Requisitos Técnicos
1. `Cargo.toml`:
   - `opt-level = 3`.
   - `lto = "fat"`.
   - `codegen-units = 1`.
   - `panic = "abort"`.
   - `strip = true`.
2. `.cargo/config.toml`:
   - Configurar `rustflags = ["-C", "target-feature=+crt-static"]` para os targets `x86_64-unknown-linux-musl` e `aarch64-unknown-linux-musl`.
