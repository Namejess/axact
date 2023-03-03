# Moniteur CPU Htop-Like

Ce projet est un moniteur de CPU Htop-Like, écrit en Rust et utilisant Axum et Tokio pour la gestion des requêtes HTTP. Le moniteur CPU est basé sur la crate sysinfo, qui fournit des informations sur l'utilisation du CPU et de la mémoire.

Le front-end est développé en Preact et est accessible via une interface web. Le front-end récupère les données de la partie serveur et les affiche dans une interface utilisateur simple et intuitive.

## Installation

1. Clonez le dépôt Git sur votre machine.
2. Dans le répertoire racine du projet, exécutez la commande `cargo build` pour construire le projet.
3. Exécutez la commande `npm install` pour installer les dépendances front-end.
4. Exécutez la commande `npm run build` pour construire le front-end.
5. Exécutez la commande `cargo run` pour lancer le serveur.
6. Ouvrez votre navigateur web et accédez à l'URL `http://localhost:8081` pour accéder à l'interface utilisateur.

## Utilisation

Lorsque vous accédez à l'interface utilisateur, vous verrez une liste des processus actifs et leur utilisation du CPU en temps réel.
