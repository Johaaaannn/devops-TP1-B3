# devops-TP1-B3

Rust Ping API

📌 Description

Cette API en Rust utilise actix-web et retourne les headers de la requête en JSON lorsqu'on envoie une requête GET sur /ping.

🚀 Installation & Exécution

Cloner le repo

```bash
git clone https://github.com/votre-utilisateur/rust-ping-api.git
cd rust-ping-api
```

Installer Rust (si ce n'est pas fait)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Ajouter les dépendances

```bash
cargo build
```

Lancer le serveur (par défaut sur le port 8080)

```bash
cargo run
```

Ou avec un port personnalisé :

```bash
PING_LISTEN_PORT=3000 cargo run
```

🔍 Test de l'API

Faire une requête GET sur /ping :

```bash
curl -X GET http://127.0.0.1:8080/ping
```

Exemple de réponse :
```json
{
  "host": "127.0.0.1:8080",
  "user-agent": "curl/7.81.0",
  "accept": "*/*"
}
```
