Moteur de jeu voxel avec pour objectif de reproduire un Minecraft basique et d'y implémenter de nombreuses mécaniques d'automatisation (en s'inspirant de factorio, satisfactory et feed the beast)
Le tout dans un monde infinie dans les 3 dimensions (Minecraft n'est pas infinie en hauteur/profondeur)

Le moteur est codé en Rust afin de privilégier la vitesse de génération du monde et avoir une meilleurs gestion des leaks et du thread safe. (et également pour découvrir le Rust)

Le projet étant en cours il n'y a pas encore vraiment de gameplay et de nombreuses optis sont encore à faire.

Pour lancer le projet:
- Installer Rust
- Cloner le projet
- Dans le dossier du projet: cargo run --release

- Déplacement avec ZQSD LShift et Espace


Optimisations principales actuellement déployées:
- Greedy Meshing
- MultiThreading

Features actuellement déployées:
- Ambient Occlusion
- Génération de structures (des arbres uniquement pour le moment)
- Lumière naturelle et diffuse inter-chunks
- Génération de monde basique avec Perlin2D pour la surface et Perlin3D pour les caves
