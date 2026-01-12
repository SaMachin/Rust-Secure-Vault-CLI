# Rust Secure Vault CLI
Un gestionnaire de secrets en ligne de commande (CLI) écrit en Rust. Ce projet permet de stocker des notes chiffrées dans un fichier local en se basant sur des normes cryptographiques modernes.

## Caractéristiques
- **Faible couplage et modularité** : L'architecture est divisée en modules indépendants, ce qui permet de modifier la cryptographie ou le mode de stockage (ex: passer de JSON à BIN) sans impacter le reste du système.

- **Chiffrement Authentifié (AEAD)** : Utilisation d'AES-256-GCM pour garantir la confidentialité et l'intégrité des données.
  
- **Dérivation de Clé** : Implémentation d'Argon2id avec un salt aléatoire pour chaque entrée, ce qui permet une résistance optimale aux attaques par brute force.

- **Gestion des Erreurs** : Utilisation de anyhow pour chaîner les contexts d'erreurs, ce qui permet de remonter la trace du problème en même temps de donner des messages clairs à l'utilisateur.

## Pile Technique
- **Langage** : Rust

- **CLI** : clap avec sous-commandes

- **Cryptographie** : aes-gcm, argon2, rand (nonce)

- **Sérialisation** : serde et serde_json avec encodage Base64 (serde_with)

- **Sécurité de saisie** : rpassword pour masquer les mots de passe dans le terminal

## Utilisation
#### **Ajouter une entrée**  
Dans le cas où l'entrée existe déjà, le programme va afficher un avertissement et demander une confirmation avant d'écraser l'entrée déjà existante.
```
add <label>
```
#### **Lire une entrée**
```
read <label>
```
#### **Supprimer une entrée**
Contrairement aux autres commandes, delete ne requiert pas le mot de passe fourni lors de la création de l'entrée, étant donné que les secrets sont enregistrés localement sur le disque. Ainsi, il est très facile de directement supprimer le fichier de secrets sans passer le terminal. Par conséquent, demander le mot de passe avant de supprimer une entrée serait une "couche de sécurité" placebo.
```
delete <label>
```
#### **Message d'aide**
```
help [command]
```

## Architecture du Code
- **src/main.rs** : Point d'entrée gérant les modules et l'ordre logique du programme.

- **src/cli.rs** : Parsing et définition des commandes.

- **src/crypto.rs** : Chiffrement/déchiffrement sans dépendance aux fichiers.

- **src/json_vault.rs** : Gestion du vault en format JSON.

## Roadmap
- **Paramètres Argon2 personnalisables** : Permettre à l'utilisateur de configurer le coût mémoire et temporel de la dérivation de clé.

- **Modification d'entrées** : Permettre la modification d'une entrée au lieu de devoir l'écraser.

- **Multi-notes** : Permettre d'avoir plus d'un message par entrée.

- **Support multi-vault** : Permettre la création et la gestion de plusieurs fichiers de coffre-fort indépendants.

## Ce que j'ai appris
- **Authenticité (AEAD)** : Utilisation d'AES-256-GCM pour garantir que les secrets n'ont pas été altérés ou corrompus sur le disque.

- **Dérivation de clé** : Utilisation d'Argon2 pour transformer un mot de passe en une clé résistante aux attaques par force brute.

- **Unicité cryptographique** : Génération de nonces et de salts aléatoires pour assurer un chiffrement unique à chaque fois.
  
- **Gestion des arguments** : Capture des arguments comme le label directement depuis la ligne de commande.

- **Crate Serde** : Sérialisation et désérialisation de structures de données vers le format JSON en Rust.

- **Gestion des erreurs avec anyhow** : Simplification de la propagation d'erreurs et transformation des échecs techniques en messages clairs pour l'utilisateur.