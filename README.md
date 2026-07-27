# d CLI

> Egyszerűsített parancssoros eszköz macOS-re — IP-cím lekérdezés, git push, automatikus frissítés és még sok más.

## Áttekintés

A `d` egy parancssoros segédeszköz, amely összevonja a mindennapi fejlesztői feladatokat: helyi IP-cím gyors lekérdezése, egy gombnyomással git commit és push, valamint automatikus frissítési rendszer.

## Telepítés

### MacOS

```bash
curl -fsSL https://dcli.dezso.hu/install.sh | bash
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
| `d gen hex` | Véletlenszerű hex (`openssl rand -hex 32`) |
| `d gen hex 64` | Hex megadott hosszal (`openssl rand -hex 64`) |
| `d gen base64` | Véletlenszerű base64 (`openssl rand -base64 32`) |
| `d gen base64 64` | Base64 megadott hosszal (`openssl rand -base64 64`) |
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

### Hex / base64 generálása

```bash
d gen hex
d gen hex 64
d gen base64
d gen base64 64
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

- **Verzió:** 0.4.3
- **Szerző:** Dezső Benedek
- **Dokumentáció:** [GitHub repo](https://github.com/DezBenedek/d)
- **Licenc:** MIT

---

_Eszköz összevonja a gyakori feladatokat egyetlen parancssori felületbe, hogy a fejlesztők kevesebbet pápoljanak, többet dolgozzanak._
