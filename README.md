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

