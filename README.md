# d CLI

> Egyszerűsített parancssoros eszköz macOS-re — IP-cím lekérdezés, git push, automatikus frissítés és még sok más.

## Áttekintés

A `d` egy parancssoros segédeszköz, amely összevonja a mindennapi fejlesztői feladatokat: helyi IP-cím gyors lekérdezése, egy gombnyomással git commit és push, valamint automatikus frissítési rendszer.

## Telepítés

### MacOS

```bash
curl -fsSL https://dcli.dezso.hu/install.sh | sh
```

Telepítés után ellenőrizd a működést:

```bash
d --help
```

## Parancsok

| Parancs | Leírás |
|---------|--------|
| `d ip` | A gép helyi (LAN) IP-címének kiírása |
| `d push "üzenet"` | `git add -A` + commit + push az aktuális branch-re |
| `d update` | A legújabb verzió letöltése és telepítése GitHub-ról |
| `d git fix` | `.gitignore`-ban tiltott, de már trackelt fájlok eltávolítása a git indexből |
| `d git update` | Git-hez kapcsolódó frissítési művelet |
| `--authors` | A szerző nevének kiírása |
| `--doc` | A dokumentáció linkjének kiírása |

## Példák

### Helyi IP-cím lekérdezése

```bash
d ip
```

### Gyors commit és push

```bash
d push "új funkció hozzáadva"
```

### Frissítés a legújabb verzióra

```bash
d update
```

### Git index tisztítása

```bash
d git fix
```

## Fejlesztés

A projekt Rust nyelven íródott, és a [`clap`](https://crates.io/crates/clap) könyvtárat használja parancssor-feldolgozásra.

```bash
# Projekt fordítása
cargo build --release

# Futtatás fejlesztési módban
cargo run -- --help

# Csomag csomagolása
./build-pkg.sh
```

## Információk

- **Verzió:** 0.4.0
- **Szerző:** Dezső Benedek
- **Dokumentáció:** [GitHub repo](https://github.com/DezBenedek/d)
- **Licenc:** MIT

---

_Eszköz összevonja a gyakori feladatokat egyetlen parancssori felületbe, hogy a fejlesztők kevesebbet pápoljanak, többet dolgozzanak._
