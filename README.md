# Axum-Mongo-API

Axum API for CRUD operations in MongoDB.

## 🛠 Built With

[![Rust](https://img.shields.io/badge/rust-firebrick?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-firebrick?style=for-the-badge&logo=rust&logoColor=white)](https://docs.rs/axum/latest/axum/)
[![MongoDB](https://img.shields.io/badge/mongodb-forestgreen?style=for-the-badge&logo=mongodb&logoColor=white)](https://www.mongodb.com/)
[![Docker](https://img.shields.io/badge/docker-navy?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

## ⚙️ Run Locally

Clone the project

```bash
git clone https://github.com/tr1sm0s1n/axum-mongo-api.git
cd axum-mongo-api
```

Install Rust

```bash
make install
```

Start the database

```bash
make up
```

Run the application

```bash
make run
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
make enter
```

Stop the database

```bash
make down
```
