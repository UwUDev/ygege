# Support TMDB et IMDB 

Cette documentation fournit des informations sur la façon d'activer et d'utiliser le support TMDB et IMDB dans Ygégé.

## Activation du support TMDB et IMDB

Pour activer le support TMDB et IMDB, vous devez définir la variable d'environnement appropriée `TMDB_TOKEN` avec votre jeton API TMDB *(celui-ci jwt)* ou l'ajouter à votre fichier `config.json`.

## Pourquoi ai-je besoin de mon propre jeton TMDB ?

Ygégé n'est pas livré avec un jeton TMDB intégré pour éviter les problèmes potentiels d'abus et de limitation de débit. En utilisant votre propre jeton, vous vous assurez que votre utilisation est suivie sous votre compte, permettant une meilleure fiabilité et accès aux services TMDB.

## Et pour IMDB ?

Le support IMDB est géré via TMDB, donc en activant TMDB, vous aurez également accès aux informations IMDB lorsque disponibles.

## Obtenir un jeton API TMDB

1. Créez un compte sur [TMDB](https://www.themoviedb.org/signup/).
2. Connectez-vous à votre compte TMDB.
3. Créez votre jeton API en suivant les instructions sur la [documentation de l'API TMDB](https://developers.themoviedb.org/3/getting-started/introduction).
4. Remplissez le formulaire pour demander une clé API, en fournissant les détails nécessaires sur votre application.
5. Vous devriez recevoir immédiatement votre clé API, mais parfois cela peut prendre quelques minutes pour être approuvé.
6. Une fois que vous avez votre clé API, vous pouvez l'utiliser pour activer le support TMDB et IMDB dans Ygégé.

