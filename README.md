# PayMonitor AI

**PayMonitor AI** é um dashboard inteligente que detecta riscos em tempo real com base em transações on-chain da Solana.  
Utiliza Substreams para indexação de dados e machine learning (Isolation Forest) para identificar comportamentos anômalos — tudo com visualização via Next.js e integração via API.

## Estrutura

- **Substreams** para indexar eventos on-chain (transações Solana)
- **Python Flask + IsolationForest** para detectar anomalias e gerar score de risco
- **Next.js Frontend** consumindo API `/risk` e exibindo resultados
- **Docker Compose** para rodar backend e frontend integrados

## Como rodar

```bash
# 1. Clone o repositório
git clone github.com/PagCrypto/paymonitor-ai && cd paymonitor-ai

# 2. Construa e inicie os serviços
docker-compose up --build
```

Acesse `http://localhost:3000` para visualizar o dashboard.