# PagMonitor AI

Dashboard inteligente para análise de risco em transações Solana com Substreams, IA e visualização em tempo real para validadores e usuários da PagCrypto.

## Estrutura

- **Substreams** para indexar eventos on-chain (transações Solana)
- **Python Flask + IsolationForest** para detectar anomalias e gerar score de risco
- **Next.js Frontend** consumindo API `/risk` e exibindo resultados
- **Docker Compose** para rodar backend e frontend integrados

## Como rodar

```bash
# 1. Clone o repositório
git clone <REPO_URL> && cd pagmonitor-ai

# 2. Construa e inicie os serviços
docker-compose up --build
```

Acesse `http://localhost:3000` para visualizar o dashboard.