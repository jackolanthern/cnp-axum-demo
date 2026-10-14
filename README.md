# cnp-axum-demo

Petite application web en **Rust / Axum** pour tester la génération de fichiers de
déploiement de CNP sur un framework non prévu par la plateforme (POC 2-399).

## Lancer

```sh
cargo run --release
```

Le serveur écoute sur `0.0.0.0` et le port de la variable `PORT` (8080 par défaut).

| Route | Réponse |
|---|---|
| `/` | Page HTML |
| `/health` | `{"status":"ok"}` |
| `/api/info` | Service, langage, framework et version |

Aucune base de données ni variable secrète n'est nécessaire.
