# Isometric Terrain 2D

Un exemple simple de rendu de terrain 2D isométrique avec Bevy.

## Fonctionnalités

- Rendu de terrain isométrique avec différents types de tuiles
- Génération procédurale de terrain (herbe, eau, sable, montagne)
- Déplacement de la caméra avec WASD ou les flèches
- Zoom avec la molette de la souris
- Affichage d'informations à l'écran

## Exécution

```bash
cd examples/isometric_terrain_2d
cargo run
```

## Contrôles

- **WASD / Flèches** : Déplacer la caméra
- **Molette de la souris** : Zoomer / Dézoomer

## Structure du code

- `main.rs` : Point d'entrée avec toute la logique
  - `TerrainType` : Enum pour les différents types de terrain
  - `generate_terrain()` : Génère un terrain 20x15 avec différents types
  - `isometric_to_world()` : Convertit les coordonnées isométriques en coordonnées monde
  - `setup()` : Initialise la caméra et les tuiles du terrain
  - `camera_movement()` : Gère le déplacement de la caméra

## Configuration

- **Taille du terrain** : 20x15 tuiles
- **Taille des tuiles** : 64x64 pixels
- **Types de terrain** : Herbe (vert), Eau (bleu), Sable (jaune), Montagne (gris)

## Dépendances

- Bevy 0.19 (dernière version stable)
- Aucune dépendance externe supplémentaire

## Améliorations possibles

- [ ] Ajouter des sprites pour les tuiles au lieu de rectangles colorés
- [ ] Implémenter un système de chunks pour les grands terrains
- [ ] Ajouter des animations (eau, vent dans l'herbe)
- [ ] Implémenter la sélection de tuiles avec la souris
- [ ] Ajouter un système de pathfinding (A*)
- [ ] Implémenter le défilement infini du terrain
- [ ] Ajouter des bâtiments et des unités
- [ ] Sauvegarder/Charger le terrain
- [ ] Ajouter des tests unitaires pour la logique
