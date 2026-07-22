# zkp-auth-demo

Rust microservice demo for a bachelor thesis on Zero-Knowledge-Proof-based authentication between services.

The project models a small e-commerce flow with three services:

- `order-service` receives client orders and orchestrates the flow.
- `warehouse-service` manages inventory and reserves products.
- `payment-service` simulates payment approval.

Final version should integrate `zkp-auth-lib` so that internal service-to-service calls are authenticated with Zero-Knowledge Proofs instead of static API keys or shared secrets.

## Run Locally

```bash
bash scripts/run-services.sh
```

Then test the full flow:

```bash
curl -X POST http://127.0.0.1:3000/orders \
  -H "content-type: application/json" \
  -d '{"customer_id":"customer-1","product_id":"laptop","quantity":1,"payment_method":"demo-card"}'
```

Default ports:

- Order Service: `http://127.0.0.1:3000`
- Warehouse Service: `http://127.0.0.1:3001`
- Payment Service: `http://127.0.0.1:3002`
