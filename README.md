# constello

Outil de gestion des transactions utilisé pour [constellation.lacontrevoie.fr](https://constellation.lacontrevoie.fr).

**Fonctionnalités :**
- Création d’une constellation participative : nos donateur·ices peuvent placer des étoiles sur un ciel étoilé. Ça fera une belle image à la fin de la campagne de dons !
- S’interface avec l’API de HelloAsso (callback) pour recevoir les transactions en temps réel
- Envoi d’emails de remerciement de dons / adhésions de manière semi-automatique avec des templates prédéfinis
- Interface d’administration où sont listées toutes les transactions reçues, avec possibilité de les cocher / archiver une fois saisies en compta
- API CRUD pour gérer les transactions et les étoiles
- BDD PostgreSQL ou SQLite

**Technique :**
- Frontend en vanilla JS, HTML et CSS sans framework
- Backend en Rust avec [Actix](https://actix.rs/) et [Diesel](https://diesel.rs) pour la DB
- ~1 500 lignes de Rust, 700 lignes de JS

Vous pouvez contribuer au projet en regardant les [tickets ouverts](https://git.42l.fr/42l/constello/issues).

### Déploiement

#### Avec Docker

Télécharger l’image :

```
docker pull git.lacontrevoie.fr/lacontrevoie/constello:latest
```

L’image proposée intègre uniquement le support PostgreSQL.

#### Manuel

- Clôner le dépôt :

```sh
git clone https://git.42l.fr/42l/constello # ou ssh://git@git.42l.fr:42084/42l/constello.git
```

- Compiler le serveur selon le backend de la base de données.

Pour PostgreSQL :
```sh
cargo build
```

Pour SQLite :

```sh
cargo build --no-default-features --features sqlite
```

### Configuration


- Copier `config.toml.sample` vers `config.toml` et configurer l’instance. Éventuellement modifier les templates par défaut.
- Démarrer le serveur. Le binaire `constello` peut être extrait du dossier `target`, tant qu’il se situe à la racine du dépôt.
- En production : mettre l’application derrière un reverse-proxy et restreindre l’accès aux routes `/admin/` avec une *basic auth* (obligatoire)
