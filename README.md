# Raust

## Description

Raust est un petit gestionnaire de mots de passe écrit entièrement en Rust. Il vise à fournir une base simple, sûre et auditable pour stocker et récupérer des identifiants chiffrés localement. Ce dépôt contient le code source minimal nécessaire pour un gestionnaire UI léger

## Fonctionnalités

- Stockage local chiffré des entrées (nom, identifiant, mot de passe).
- UI simple pour ajouter, récupérer et lister des entrées.
- Code écrit en Rust, sans dépendances externes lourdes quand possible.

## Compilation

Depuis la racine du projet :

```bash
cargo build --release
```

Le binaire sera généré dans `target/release/`.


## Modèle de sécurité (résumé)

- Les secrets sont chiffrés avant d'être écrits sur disque.
- La clé de chiffrement doit être dérivée d'une passphrase ou stockée de façon sûre (clé maître). Le projet vise l'utilisation d'APIs de dérivation de clé (scrypt/argon2) pour limiter les attaques par force brute.
- Ne synchronisez pas le fichier chiffré sur un service non fiable sans comprendre les risques.

## Structure du dépôt

- `Cargo.toml` — manifeste Cargo
- `src/main.rs` — point d'entrée de l'application
- `src/core/crypto.rs` — fonctions de chiffrement/déchiffrement
- `src/core/storage.rs` — lecture/écriture du magasin chiffré
- `src/core/models.rs` — modèles de données (entrée de mot de passe, métadonnées)

## Contribution

Les contributions sont bienvenues. Ouvrez une issue pour discuter d'abord des changements importants et créez des PRs ciblées. Privilégiez des commits clairs et des tests pour les nouvelles fonctionnalités.

## Points à améliorer / TODO

- Documenter précisément le code.
- Ajouter des tests d'intégration pour valider les scénarios d'initialisation, import/export et migration de format.
- Ajouter une option d'export chiffré compatible avec d'autres gestionnaires.