

<br>

# Prerequisites

- **Rust** (latest stable version) - (https://rustup.rs/)
- **wasm-pack** - Install with:
  ```bash
  cargo install wasm-pack
  ```
  
### Build

1. **WebAssembly:**
   ```bash
   wasm-pack build --target web
   ```

2. **Local web server:**
   ```bash
   python -m http.server 8000
   ```

  `http://localhost:8000` in your browser.

<br>

# Game Rules

- **Placement Phase**: Players take turns placing one piece on any vacant square until all 16 pieces (8 per player) are placed on the board
- **Starting Player**: Light player places first

### Movement Phase

After all pieces are placed, players take turns moving one piece per turn.

**Basic Movement:**
- Pieces move one square orthogonally (horizontally or vertically)
- Pieces cannot move diagonally
- Pieces cannot move through other pieces

**Jumping:**
- A piece can jump over an adjacent enemy piece if the square immediately beyond is empty
- Similar to draughts/checkers jumping mechanics
- Multiple jumps are allowed in a single turn if possible
- You must continue jumping if additional jumps are available

### Capturing

- A piece is captured when it is surrounded on two opposite sides (orthogonally) by enemy pieces
- The two enemy pieces must be directly adjacent to the captured piece on opposite sides (north-south or east-west)
- Captured pieces are removed from the board immediately
- Captures can occur during either player's turn when the condition is met

### Winning the Game

- **Victory**: Capture all of your opponent's pieces, OR immobilize them so they cannot make any legal moves
- **End**: The game ends immediately when a player has no pieces remaining or cannot move.

<br>

# Historical

- **Earliest References**: The game is mentioned in Roman literature, including works by Ovid and Varro
- **Archaeological Evidence**: Game boards and pieces have been found at Roman sites across Europe, from Britain to the Middle East
- **Cultural Significance**: The game was played by all social classes, from soldiers to senators, and was considered a game of skill and strategy

### Game Characteristics

Latrones shares similarities with modern chess and draughts:

- **Strategic Depth**: Requires planning, tactical thinking, and positional play
- **Asymmetric Warfare**: Players must balance offense (capturing) and defense (protecting pieces)
- **Spatial Reasoning**: Success depends on understanding board geometry and piece relationships

### Rule Reconstruction Challenge

Unlike some ancient games, the exact rules of Latrones were not fully documented, leading to multiple modern reconstructions:

- **Schädler's Reconstruction**: Based on archaeological evidence and comparative game analysis, emphasizing piece placement and orthogonal movement
- **Museum Quintana Reconstruction**: Alternative interpretation with rook-like movement patterns

#### Why the rules weren't fully documented

1. **Oral tradition and informal transmission**
   - Rules were passed down orally or by demonstration, not written down
   - Common for recreational activities in ancient cultures
   - Details were lost over time as the tradition faded

2. **Limited literacy and documentation priorities**
   - Literacy was not widespread in the Roman Empire
   - Writing was expensive and often reserved for official, legal, or literary works
   - Games were seen as everyday recreation, not worth documenting in detail

#### Why literacy wasn't widespread in the Roman Empire

1. **Social stratification and class barriers**
   - Education was mainly for the elite (wealthy families, aristocrats, senators)
   - Lower classes (peasants, slaves, laborers) had little or no access to formal education
   - Literacy was a status marker, not a universal skill

2. **Economic constraints**
   - Education was expensive: private tutors, writing materials (papyrus, ink, stylus), and time away from work
   - Most people couldn't afford to have children not working
   - Education was a private expense, not publicly funded


#### How Roman Emperors & Governors Communicated with Non-Roman 

1. **Multilingual administration**
   - Greek as lingua franca: In the eastern provinces, Koine Greek was widely used alongside Latin. Many Roman officials were bilingual.
   - Local languages: Officials used interpreters for languages like Punic (North Africa), Aramaic (Middle East), Celtic (Gaul/Britain), and Germanic languages.
   - Bilingual inscriptions: Public decrees were often posted in both Latin and the local language.
   - Pragmatic multilingualism: The administration adapted to local languages rather than forcing Latin on everyone.
   - This flexible, multilingual approach helped the empire govern diverse populations across vast territories, even with limited literacy.    

2. **Interpreters and translators**
   - Professional interpreters were used for legal proceedings, administrative meetings, and official communications.
   - Local bilingual elites often served as intermediaries, bridging Roman administration and local communities.
   - Hierarchical transmission: Messages flowed: Emperor → Governor → Local oligarchy → Common people.

3. **Cursus Publicus (state courier system)**
   - Established by Augustus, this network of relay stations enabled rapid communication across the empire.
   - Couriers carried messages, decrees, and official documents to provincial centers.
   - Ensured consistent communication from the center to the provinces.

4. **Local intermediaries and elites**
   - Governors relied on local leaders, tribal chiefs, and provincial elites who understood both Roman ways and local customs.
   - These intermediaries translated and explained Roman policies to their communities.
   - Often served as local administrators (e.g., client kings, local magistrates).

5. **Public decrees and inscriptions**
   - Laws, edicts, and imperial messages were carved on stone and displayed in public places.
   - Often bilingual (Latin + local language) or multilingual.
   - Visible in forums, temples, and city gates.

### Legacy

Latrones influenced the development of later European strategy games and is considered a precursor to games like:
- Medieval Tafl games
- Modern chess (in terms of strategic)
- Draughts/checkers (in terms of jumping mechanics)

<br>

# Structure

```
.
├── .gitignore               # Git ignore rules                 (Config)
├── Cargo.toml               # Rust project configuration       (Backend)  (Config)
├── Cargo.lock               # Rust dependency lock file        (Backend)  (Config)
├── package.json             # Node.js project metadata         (Frontend) (Config)
├── index.html               # HTML entry point                 (Frontend) (Static / 1 Markup)
├── src/
│   ├── lib.rs               # Rust game logic + rendering      (Backend)  (Source / 2 Library)
│   │                         # - GameState struct & methods
│   │                         # - Game rules & validation
│   │                         # - HTML rendering (render_game_info_html, render_game_board_html)
│   ├── main.js              # JavaScript thin client           (Frontend) (Source / 6 Script)
│   │                         # - WASM initialization
│   │                         # - DOM updates only
│   │                         # - Event handler wrappers
│   ├── App.css              # Application styles               (Frontend) (Static / 4 Styles)
│   ├── index.css            # Global styles                    (Frontend) (Static / 4 Styles)
│   └── components/          # Component styles
│       ├── GameBoard.css    # Board styling                    (Frontend) (Static / 4 Styles)
│       └── GameInfo.css     # Info component styling           (Frontend) (Static / 4 Styles)
├── pkg/                     # wasm-pack generated (gitignored) (Backend)
│   ├── latrones.js          # WASM bindings                    (Backend)  (Source / 3 Module)
│   ├── latrones_bg.wasm     # Compiled WebAssembly             (Backend)  (Source / 2 Library)
│   ├── latrones.d.ts        # TypeScript definitions           (Backend)  (Source / 3 Module)
│   ├── latrones_bg.wasm.d.ts # WASM TypeScript definitions     (Backend)  (Source / 3 Module)
│   └── package.json         # WASM package metadata            (Backend)  (Config)
└── README.md                # This file
```