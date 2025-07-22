# Axum-Mongo-API

Axum API for CRUD operations in MongoDB.

## 🛠 Built With

[![Rust Badge](https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=for-the-badge)](https://www.rust-lang.org/)
[![Axum Badge](https://img.shields.io/badge/Axum-000?logo=rust&logoColor=fff&style=for-the-badge)](https://docs.rs/axum/latest/axum/)
[![MongoDB Badge](https://img.shields.io/badge/MongoDB-47A248?logo=mongodb&logoColor=fff&style=for-the-badge)](https://www.mongodb.com/)
[![Docker Badge](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff&style=for-the-badge)](https://www.docker.com/)

## ⚙️ Run Locally

Clone the project

```bash
git clone https://github.com/tr1sm0s1n/axum-mongo-api.git
cd axum-mongo-api
```

Install Rust

```bash
just install
```

Start the database

```bash
just up
```

Run the application

```bash
just run
```

Create a certificate

```bash
curl -X POST http://127.0.0.1:8080/create -H "Content-Type: application/json" -d '{"_id": 1, "name": "Langley", "course": "9A"}'
```

Read all certificates

```bash
curl http://127.0.0.1:8080/read
```

Read a certificate

```bash
curl http://127.0.0.1:8080/read/1
```

Update a certificate

```bash
curl -X PUT http://127.0.0.1:8080/update/1 -H "Content-Type: application/json" -d '{"_id": 1, "name": "Nightingale", "course": "MBCC"}'
```

Delete a certificate

```bash
curl -X DELETE http://127.0.0.1:8080/delete/1
```

View the database (optional)

```bash
just enter
```

Stop the database

```bash
just down
```
