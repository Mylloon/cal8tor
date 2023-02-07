# cal8tor • *cal*endar P*8* extrac*tor*

Extracteur d'emploi du temps pour la licence d'informatique de Paris 8

[![dependency status](https://deps.rs/repo/gitea/git.mylloon.fr/Anri/cal8tor/status.svg)](https://deps.rs/repo/gitea/git.mylloon.fr/Anri/cal8tor)

## Installation

### Arch

cal8tor est disponible sur le AUR : [`cal8tor`](https://aur.archlinux.org/packages/cal8tor)
et [`cal8tor-git`](https://aur.archlinux.org/packages/cal8tor-git).

### Manuellement

Cf. [Compilation et installation](#compilation-et-installation).

## Lancer

Pour afficher la page d'aide

```
$ cal8tor --help
```

## Voir le calendrier dans le terminal

Pour les L2-X par exemple, lance :

```bash
$ cal8tor l2-X
```

> Le rendu peut parfois être difficile à lire, n'hésites pas à utiliser l'option
> `-c` (ou `--cl`) pour ajuster la longueur des cellules du planning.

## Exporter le calendrier au format `.ics`

Pour les L1-A par exemple, lance :

```bash
$ cal8tor L1A --export calendar.ics
```

> Le fichier comprend le fuseau horaire pour `Europe/Paris` et est
> conforme à [cet outil de validation](https://icalendar.org/validator.html).

## Compilation et installation

Vous aurez besoin de Rust pour compiler le programme.

<details><summary>Vous avez aussi besoin d'<code>OpenSSL</code>.</summary>

- Ubuntu: `sudo apt install libssl-dev`
- Fedora: `dnf install openssl-devel`
</details>

1. Clone le dépôt et s'y rendre

```bash
$ git clone https://git.mylloon.fr/Anri/cal8tor.git && cd cal8tor
```

2. Compiler et installer l'application

```bash
$ cargo install --path .
```

3. Tu peux maintenant supprimer le dossier `cal8tor` !

---

N'hésite pas à faire un PR pour améliorer le projet !
