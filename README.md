# bevy-experiences

Ma collection personnelle de tests et d'exemples du framework Bevy et des crates associées.

## Organisation

Ce dépôt est organisé en sous-dossiers, chaque dossier dans `examples/` contient un exemple autonome avec son propre `Cargo.toml`.

Pour exécuter un exemple :
```bash
cd examples/<nom_de_l_exemple>
cargo run
```

## Exemples disponibles

| Exemple | Description | Bevy | Statut |
|---------|-------------|------|--------|
| [isometric_terrain_2d](./examples/isometric_terrain_2d) | Rendu de terrain 2D isométrique avec différents types de tuiles | 0.19 | ✅ Complet |
| [move_player](./examples/move_player) | Déplacement de personnage 2D avec animations Aseprite (bevy_aseprite_ultra) | 0.19 | ✅ Complet |

## Structure du projet

```
bevy-experiences/
├── Cargo.toml                 # Workspace Cargo (Bevy 0.19)
├── README.md                 # Ce fichier
└── examples/
    ├── isometric_terrain_2d/
    │   ├── Cargo.toml
    │   ├── src/
    │   │   └── main.rs
    │   └── README.md
    ├── move_player/
    │   ├── Cargo.toml
    │   ├── src/
    │   │   └── main.rs
    │   └── assets/
    │       └── player.aseprite
    └── <futur_exemple>/
        ├── Cargo.toml
        ├── src/
        │   └── main.rs
        └── README.md
```

## Objectifs

- ✅ Créer une collection d'exemples Bevy
- ✅ Organiser chaque exemple dans son propre dossier
- ✅ Permettre l'exécution directe avec `cargo run`
- 🔄 Ajouter des tests unitaires pour chaque exemple
- 🔄 Documenter les retours d'expérience
- 🔄 Explorer différentes crates compatibles avec Bevy

## Contribution

Chaque exemple doit :
1. Avoir son propre dossier dans `examples/`
2. Contenir un `Cargo.toml` valide avec Bevy 0.19
3. Avoir un `main.rs` fonctionnel
4. Inclure un `README.md` avec la description et les contrôles
5. Être exécutable avec `cargo run` depuis son dossier

## Ressources utiles

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Bevy GitHub](https://github.com/bevyengine/bevy)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- [Bevy Asset Library](https://bevyassets.com/)

## Configuration recommandée

Pour exécuter les exemples sur ta machine, assure-toi d'avoir :
- Rust 1.70+ (recommandé : dernière version stable)
- Les dépendances système pour Bevy (winit, wayland/x11, etc.)

Sur Linux (Debian/Ubuntu) :
```bash
sudo apt install pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

Sur macOS :
```bash
brew install pkg-config
```

Sur Windows : Les dépendances sont généralement gérées automatiquement par Cargo.
