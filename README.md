# Backend de Clínica Odontológica com WASM

Este projeto implementa um backend para uma clínica odontológica utilizando Rust com Axum como servidor HTTP e módulos WASM para a lógica de negócios.

## Estrutura do Projeto

```
/
├── Cargo.toml (workspace)
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── wasm_runtime.rs
│       └── routes/
│           ├── patients.rs
│           ├── appointments.rs
│           └── payments.rs
└── wasm_modules/
    ├── patients/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── appointments/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    └── payments/
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

## Funcionalidades

- `/api/patients` - Gerenciamento de pacientes
- `/api/appointments` - Gerenciamento de agendamentos
- `/api/payments` - Gerenciamento de pagamentos

Cada endpoint chama módulos WASM correspondentes para processamento da lógica de negócios.

## Requisitos

- Rust (última versão estável)
- Target `wasm32-unknown-unknown`

## Configuração

1. Instale o Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

2. Adicione o target WASM:
```bash
rustup target add wasm32-unknown-unknown
```

3. Compile os módulos WASM:
```bash
cd wasm_modules/patients && cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/patients.wasm ../../wasm_modules/patients.wasm

cd ../appointments && cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/appointments.wasm ../../wasm_modules/appointments.wasm

cd ../payments && cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/payments.wasm ../../wasm_modules/payments.wasm
```

4. Execute o backend:
```bash
cd ../.. # voltar ao diretório raiz
cargo run
```

O servidor iniciará na porta 3000.

## Endpoints da API

- `GET /api/patients` - Lista todos os pacientes
- `POST /api/patients` - Cria um novo paciente
- `GET /api/appointments` - Lista todos os agendamentos
- `POST /api/appointments` - Cria um novo agendamento
- `GET /api/payments` - Lista todos os pagamentos
- `POST /api/payments` - Cria um novo pagamento

## Observações

- Os módulos WASM contêm lógica mockada para fins de demonstração
- O backend atua como orquestrador, carregando e executando os módulos WASM de forma isolada
- A comunicação entre o backend e os módulos WASM é feita via JSON