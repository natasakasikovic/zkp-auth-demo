# zkp-auth-demo

Rust microservice demo for a bachelor thesis on Zero-Knowledge-Proof-based authentication between services.

The project models a small e-commerce flow with three services:

- `order-service` receives client orders and orchestrates the flow.
- `warehouse-service` manages inventory and reserves products.
- `payment-service` simulates payment approval.

Internal service-to-service calls are authenticated with the local `zkp-auth-lib` Schnorr proof implementation. The client still talks only to `order-service`, while `warehouse-service` and `payment-service` reject internal requests that do not contain a valid ZKP authentication proof.

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
