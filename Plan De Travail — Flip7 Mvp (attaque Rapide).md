# Plan d'attaque — MVP Flip7 (React Native + Rust)

> Ce document liste un plan d'attaque pratique pour construire **le MVP le plus rapidement possible**, avec une liste d'issues / tâches actionnables que tu pourras créer directement dans GitHub. L'objectif : avoir une **version jouable** (local + P2P basique) qui montre la boucle de gameplay, le réseau host/client, la pause/reprise et les scores.

---

## MVP — périmètre

**Fonctionnalités indispensables (MUST)**

* Partie multijoueur P2P avec **host** (host local / joueur) et jusqu'à **4 joueurs**.
* Règles de base Flip7 : pioche/stop, bust (duplicate), Flip7 (7 cartes différentes), comptage des points, rounds.
* UI minimale : écran titre, lobby (créer/join), écran de jeu (main des joueurs, pile, actions hit/stay), scoreboard.
* Pause / reprise de la partie pour le host (etat synchronisé).
* Logique du jeu en **Rust** (lib réutilisable) : règles, RNG, sérialisation d'état, messages réseau.
* Communication React Native ↔ Rust (bindings FFI ou via WebAssembly/bridge) pour la logique.
* Export sur Android/iOS (apk/ipa dev) + possibilité de build pour desktop via Electron.

**Fonctions secondaires (can/should pour MVP si reste du temps)**

* Signalisation simple (optionnel) : petit service pour découvrir l'IP ou matchmaking basique — sinon entrée manuelle d'IP/code de salle.
* Effets visuels simples (animations de flip, pop de bonus).
* Logs/network debug UI.

**Ce qui n'est pas dans le MVP**

* Boutique / monétisation / microtransactions.
* Progression complexe / comptes centralisés.
* Serveur central maintenu par toi (tout sera P2P).

---

## Architecture prompte (haute niveau)

* **Frontend** : React Native (Typescript) — UI, animations, screens, audio.
* **Logique & Netcode** : Rust (crate `game_core`) — règles, RNG, sérialisation d'état, messages réseau.
* **Transport réseau** : P2P simple via TCP/UDP (lib Rust : `tokio` + `serde` pour sérialiser messages). Possibilité d'utiliser WebRTC pour mobile/web plus tard.
* **Bridge** :

  * Option A : FFI natif (Rust lib compilée en dylib / static lib, exposée via `react-native-ffi` ou module natif).
  * Option B : WebAssembly + JS wrapper (si tu pars web-first). Pour mobile natif, privilégier FFI.
* **Packaging** : React Native CLI → Android / iOS. Pour Steam: wrapper Electron pour build desktop.

---

## Stratégie d'itération (plan d'attaque en 6 étapes claires)

Chaque étape produit un incrément jouable.

### 1) Setup & PoC local (Frontend + Rust minimal)

* **But** : valider la communication RN ↔ Rust et exécuter la logique du jeu localement.
* **Tâches**:

  * [ ] Init repo `flip7-digital` + README minimal.
  * [ ] Scaffold React Native TypeScript app (`/app`).
  * [ ] Créer crate Rust `game_core` (`/rust/game_core`) avec API simple : `new_game()`, `draw_card(player)`, `stay(player)`, `get_state()`.
  * [ ] Exposer `game_core` à RN via binding minimal (ex: module natif Android/iOS ou `react-native-ffi`).
  * [ ] UI minimal : écran jeu qui appelle `game_core` et affiche état (main + pile + actions). Buttons `Hit` / `Stay` / `End Round`.
  * [ ] Tests unitaires Rust pour règles de base (bust, flip7, scoring).

### 2) Local multiplayer turn-based (single device / LAN sim)

* **But** : permettre plusieurs joueurs simulés en local (hotseat or multiple views) pour valider la logique multi-joueurs.
* **Tâches**:

  * [ ] Implémenter notion de `Player` et `Seat` dans game_core.
  * [ ] UI hotseat : changer de vue pour chaque joueur, simuler 2–4 joueurs.
  * [ ] Save/restore d'état local pour pause/reprise.

### 3) Network P2P minimal (host/client)

* **But** : faire communiquer plusieurs instances de l'app (sur la même machine via ports différents ou sur LAN) — host authoritative.
* **Tâches**:

  * [ ] Design messages réseau (JSON/CBOR) pour actions: `Join`, `StartGame`, `Draw`, `Stay`, `Pause`, `Resume`, `SyncState`.
  * [ ] Implémenter transport basique en Rust `net` crate (tokio TCP) : `host` écoute, `client` se connecte.
  * [ ] Host authoritative : seules les commandes du host peuvent valider état final et broadcast `SyncState`.
  * [ ] UI lobby : créer partie (affiche IP + port), rejoindre via IP:port ou code manuelle.
  * [ ] Gestion simple de déconnexions : si client drop, host continue ; si host drop, proposer `HostLost` (option : migration pas pour MVP).

### 4) Synchronisation & sécurité de l'état

* **But** : éviter divergence d'état et garantir la reproductibilité.
* **Tâches**:

  * [ ] Implémenter versioning d'état et checksums (SHA) pour détecter divergence.
  * [ ] All actions envoyées au host, host exécute via `game_core` et renvoie `SyncState` complet.
  * [ ] Implémenter re-synchronisation à la reconnexion (client réclame dernier état).

### 5) Polissage UI & UX minimal

* **But** : rendre le MVP agréable et compréhensible.
* **Tâches**:

  * [ ] Écran titre + règles courtes intégrées.
  * [ ] Visualisation des mains (icônes/valeurs), animations simples (flip, slide), feedback bust/flip7.
  * [ ] Timer optionnel pour tours (mode rush) — toggleable.
  * [ ] Scoreboard et fin de manche, fin de partie.

### 6) Packaging & tests cross-device

* **But** : builds testables (apk / ios dev / desktop electron) et tests en LAN.
* **Tâches**:

  * [ ] Script de build Android (debug apk).
  * [ ] Script de build iOS (dev) si tu as un Mac ou CI pour ça.
  * [ ] Docker-compose ou run scripts pour lancer plusieurs instances desktop pour test réseau.
  * [ ] Tests de robustesse réseau (latence, drop) et corrections.

---

## Liste d'issues GitHub actionnables (copiable)

> Créer ces issues dans l'ordre de priorité (1 → plus urgent). Chaque issue peut être assignée, étiquetée (`frontend` / `backend` / `network`) et mise sous milestone `MVP`.

### Priorité haute (core MVP)

* `feat(core): init Rust game_core crate with basic rules` — *backend*

  * Implémenter deck composition (0..12 counts), draw, add card to player, bust detection, flip7 detection, scoring.
* `feat(app): init React Native TypeScript app` — *frontend*

  * Scaffold app, screens minimal, navigation.
* `feat(bindings): expose game_core to RN (FFI)` — *integration*

  * Simple bridge to call `new_game`, `draw`, `stay`, `get_state`.
* `feat(ui): game screen with Hit/Stay wired to game_core` — *frontend*

  * Visualiser main, pile, boutons, logs.
* `test(rust): unit tests for core rules` — *backend*

  * Tests pour bust, flip7, scoring correctness.

### Priorité moyenne (multiplayer P2P)

* `feat(network): network message spec and wire format` — *network*

  * Définir messages et sérialisation (serde JSON/CBOR).
* `feat(net): implement host TCP server (rust)` — *network/backend*

  * Host écoute et broadcast `SyncState`.
* `feat(net): implement client connect (rust)` — *network/backend*

  * Client envoie `Join`, actions à host.
* `feat(ui): lobby screen (create/join)` — *frontend*

  * UI pour créer/join via IP:port.
* `fix(sync): authoritative host state sync` — *network*

  * Actions validées côté host puis broadcast.

### Priorité basse (polish & extras)

* `feat(ui): pause/resume button (host)` — *frontend/backend*
* `feat(ui): simple animations (flip/slide)` — *frontend/design*
* `feat(build): add dev build scripts (android/electron)` — *devops*
* `chore: setup CI lint + tests (rust + RN)` — *devops*
* `feat(net): reconnection flow for clients` — *network*

---

## Conseils pratiques & pièges à éviter

* **Keep it authoritative** : pour éviter des bugs compliqués, fais du host-authoritative : le host est la source de vérité.
* **Sérialisation complète de l'état** : au lieu d'envoyer diff complexes, envoie `SyncState` complet après chaque action majeure pour MVP (plus simple et fiable).
* **RNG** : pour éviter triches, fais que le host génère les tirages et les envoie aux clients (ou utilise seed partagé + PRNG déterministe si tu veux offline determinism).
* **Start small on networking** : commence par tests locaux (même machine ports différents), puis LAN, puis réseau public testing.
* **Dev fast, polish later** : priorise la boucle jouable (hit/stay/bust/flip7/score) avant les animations.

---

## Checklist de validation MVP

* [ ] Deux clients (ou plus) peuvent se connecter au host et jouer une manche complète.
* [ ] Les busts et Flip7 sont correctement gérés et synchronisés chez tous les joueurs.
* [ ] Pause/resume fonctionne et restaure l'état pour les clients.
* [ ] Build Android debug fonctionnel + possibilité de lancer 2+ instances pour test.

---

## Template de repository (structure + fichiers clés)

Ci‑dessous un squelette de repo que tu peux créer (commande `git init` + ajouter les fichiers). Copie/colle les fichiers exemples et adapte.

```
flip7-digital/
├─ .github/
│  └─ workflows/
│     ├─ ci-rust.yml
│     └─ ci-rn.yml
├─ app/                    # React Native app (TypeScript)
│  ├─ package.json
│  ├─ tsconfig.json
│  ├─ src/
│  │  ├─ App.tsx
│  │  ├─ screens/
│  │  ├─ components/
│  │  └─ services/        # bridge / native module wrappers
│  └─ android/ ios/ ...   # natives generated by RN CLI
├─ rust/
│  ├─ game_core/          # crate principal
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  └─ net/                # crate réseau (tokio + serde)
│     ├─ Cargo.toml
│     └─ src/lib.rs
├─ scripts/
│  ├─ build-android.sh
│  ├─ run-multi-instances.sh
│  └─ build-electron.sh
├─ docs/
│  └─ arch.md
├─ .gitignore
└─ README.md
```

### Exemples de fichiers-clés (à  copier dans ton repo)

**README.md (starter)**

````markdown
# Flip7 Digital - MVP

Stack: React Native (TypeScript) + Rust (game_core & net)

## Setup local dev

### Prérequis
- Node >= 18, Yarn / npm
- Rust toolchain (stable)
- Android SDK (pour builds Android) / Xcode pour iOS

### Installer
```bash
# clone
git clone <repo>
cd flip7-digital

# frontend
cd app
yarn install

# rust
cd ../rust/game_core
cargo test
````

### Lancer l'app en dev (Android)

```bash
# depuis /app
yarn android
```

```

**.gitignore (extrait)**
```

node_modules/
android/
ios/
dist/
*.keystore
.target/
**/target/
**/*.pdb
.env
.DS_Store

````

---

## Templates CI GitHub Actions

Tu peux commencer avec deux workflows basiques : `ci-rust.yml` et `ci-rn.yml`.

### `.github/workflows/ci-rust.yml`

```yaml
name: CI - Rust
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Build game_core
        working-directory: ./rust/game_core
        run: cargo build --release
      - name: Run tests
        working-directory: ./rust/game_core
        run: cargo test --all -- --nocapture

````

### `.github/workflows/ci-rn.yml` (lint & types)

```yaml
name: CI - React Native
on: [push, pull_request]

jobs:
  node:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '18'
      - name: Install dependencies
        working-directory: ./app
        run: yarn install
      - name: Typecheck
        working-directory: ./app
        run: yarn tsc --noEmit
      - name: ESLint
        working-directory: ./app
        run: yarn eslint src --max-warnings=0

```

---

## Fichiers exemples Rust minimal

**rust/game_core/Cargo.toml (extrait)**

```toml
[package]
name = "game_core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"

[lib]
name = "game_core"
path = "src/lib.rs"
crate-type = ["cdylib", "rlib"]
```

**rust/game_core/src/lib.rs (squelette)**

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState { /* ... */ }

#[no_mangle]
pub extern "C" fn new_game() -> *mut GameState {
    // allocate and return pointer
    std::ptr::null_mut()
}
```

---

## Scripts utiles

* `scripts/run-multi-instances.sh` : script bash pour lancer plusieurs instances desktop (différents ports) afin de tester le réseau local.
* `scripts/build-android.sh` : wrapper pour lancer `cd app && yarn android --variant debug` et récupérer l'apk.
* `scripts/build-electron.sh` : script pour packager l'app RN + Electron pour tester sur desktop.

---

## Prochaine étape proposée

Souhaites-tu que je :

* **génère tous ces fichiers** (README, workflows, Cargo.toml, skeleton lib.rs, package.json minimal) et te les fournis ici en bloc pour copier-coller ?
* Ou bien que je **génère un zip** / arborescence exacte (je peux produire le contenu directement dans le chat) ?

Dis ce que tu veux que je crée maintenant et je te fournis les fichiers prêts.
