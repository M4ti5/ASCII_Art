# ASCII ART

### 🇬🇧 English

# Color ASCII Art Converter

This Rust project converts images into colored ASCII art. It adjusts the image's width to a specified value while maintaining the aspect ratio, and uses a predefined set of characters to represent different shades.

## Exemples

Dans cette section, vous pouvez voir la transformation d'une image en art ASCII coloré par notre programme. Voici des exemples d'images d'entrée et de sortie.

### Image d'Entrée

![Input Image](mimic.webp)

### Art ASCII de Sortie

La sortie est mieux visualisée dans un terminal prenant en charge la sortie colorée. Voici un aperçu de ce à quoi ressemble l'art ASCII :
![Output image](demo.png)
You can see a detailed and colored output directly in your compatible terminal by running the program.

## Requirements

- Rust
- Cargo

## Dependencies

This project uses `image` for image processing and `colored` for coloring the ASCII output. Ensure these are included in your `Cargo.toml`:

```toml
[dependencies]
image = "0.23.14"
colored = "2.1.0"
```

## Usage

To convert an image into ASCII art, run the program from the command line with the following arguments:

```bash
cargo run -- <image_path> <width>
```

### Demo Mode

To run the program in demo mode using the default image (`mimic.webp`) with a width of 125 characters, simply pass `demo` as the argument:

```bash
cargo run -- demo
```

### Using the Compiled Version

After building the project with `cargo build --release`, you'll find the executable in `target/release/`. To use it, navigate to that directory and run:

```bash
./ascii_art_converter <image_path> <width>
```

Replace `ascii_art_converter` with the actual name of your binary.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

<hr/>

### :fr: Français

# Convertisseur d'Art ASCII Coloré

Ce projet en Rust convertit des images en art ASCII coloré. Il ajuste la largeur de l'image à une valeur spécifiée tout en conservant le rapport d'aspect, et utilise un ensemble prédéfini de caractères pour représenter différentes nuances.

## Exemples

Dans cette section, vous pouvez voir la transformation d'une image en art ASCII coloré par notre programme. Voici des exemples d'images d'entrée et de sortie.

### Image d'Entrée

![Image d'Entrée](mimic.webp)

### Art ASCII de Sortie

La sortie est mieux visualisée dans un terminal prenant en charge la sortie colorée. Voici un aperçu de ce à quoi ressemble l'art ASCII :

![Image d'Entrée](demo.png)

Vous pouvez voir une sortie détaillée et colorée directement dans votre terminal compatible en exécutant le programme.

## Prérequis

- Rust
- Cargo

## Dépendances

Ce projet utilise `image` pour le traitement d'images et `colored` pour colorer la sortie ASCII. Assurez-vous qu'ils sont inclus dans votre `Cargo.toml` :

```toml
[dependencies]
image = "0.23.14"
colored = "2.1.0"
```

## Utilisation

Pour convertir une image en art ASCII, exécutez le programme depuis la ligne de commande avec les arguments suivants :

```bash
cargo run -- <chemin_vers_image> <largeur>
```

### Mode Démo

Pour exécuter le programme en mode démo en utilisant l'image par défaut (`mimic.webp`) avec une largeur de 125 caractères, passez simplement `demo` comme argument :

```bash
cargo run -- demo
```

### Utilisation de la Version Compilée

Après avoir construit le projet avec `cargo build --release`, vous trouverez l'exécutable dans `target/release/`. Pour l'utiliser, naviguez jusqu'à ce répertoire et exécutez :

```bash
./ascii_art_converter <chemin_vers_image> <largeur>
```

Remplacez `ascii_art_converter` par le nom réel de votre binaire.

## Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à soumettre une pull request.

```

Cette version inclut les drapeaux pour indiquer la langue de chaque section et ajoute des instructions sur l'utilisation de la version compilée du programme. Assurez-vous de remplacer `ascii_art_converter` par le nom réel de l'exécutable généré par votre projet.
