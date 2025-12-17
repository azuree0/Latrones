<img width="891" height="719" alt="L" src="https://github.com/user-attachments/assets/aaaf5c15-8986-4219-922d-fdffe3796a63" />

<br>

# Prerequisites

- Rust (latest stable version)
wasm-pack, install with: 
```bash
cargo install wasm-pack
```

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Load files with a local web server:
```bash
python -m http.server 8000
```

3. Open browser:
```bash
http://localhost:8000
```

# Game Rules

- Placement Phase: Players take turns placing one piece on any vacant square until all 16 pieces (8 per player) are placed on the board.
- Starting Player: Light player places first.

**Basic Movement:**
- Pieces move one square orthogonally (horizontally or vertically).
- Pieces cannot move diagonally.
- Pieces cannot move through other pieces.

**Jumping:**
- A piece can jump over an adjacent enemy piece if the square immediately beyond is empty.
- Similar to draughts/checkers jumping mechanics.
- Multiple jumps are allowed in a single turn if possible.
- You must continue jumping if additional jumps are available.

### Capturing

- A piece is captured when it is surrounded on two opposite sides (orthogonally) by enemy pieces.
- The two enemy pieces must be directly adjacent to the captured piece on opposite sides (north-south or east-west).
- Captured pieces are removed from the board immediately.
- Captures can occur during either player's turn when the condition is met.

### Winning the Game

- Victory: Capture all of your opponent's pieces, OR immobilize them so they cannot make any legal moves.
- End: The game ends immediately when a player has no pieces remaining or cannot move.

<br>

# Historical

- Earliest References: The game is mentioned in Roman literature, including works by Ovid and Varro.
- Archaeological Evidence: Game boards and pieces have been found at Roman sites across Europe, from Britain to the Middle East.
- Cultural Significance: The game was played by all social classes, from soldiers to senators, and was considered a game of skill and strategy.

### Rule Reconstruction Challenge

Unlike some ancient games, the exact rules of Latrones were not fully documented, leading to multiple modern reconstructions:

- Schädler's Reconstruction: Based on archaeological evidence and comparative game analysis, emphasizing piece placement and orthogonal movement.
- Museum Quintana Reconstruction: Alternative interpretation with rook-like movement patterns.

#### Why the rules weren't fully documented

1. **Oral tradition and informal transmission**
   - Rules were passed down orally or by demonstration, not written down.
   - Common for recreational activities in ancient cultures.
   - Details were lost over time as the tradition faded.

2. **Limited literacy and documentation priorities**
   - Literacy was not widespread in the Roman Empire.
   - Writing was expensive and often reserved for official, legal, or literary works.
   - Games were seen as everyday recreation, not worth documenting in detail.

3. **Regional variations**
   - Archaeological finds show boards of different sizes (7×9, 5×5, 11×16, 6×7, 8×8).
   - Rules likely varied by region, making a single "standard" set hard to define.
   - Players may have adapted rules locally.

4. **No preserved rulebooks**
   - No game manuals or detailed rule descriptions have survived.
   - What survives are brief literary mentions (e.g., Ovid, Varro) that reference the game but don't explain the rules.
   - Archaeological finds show boards and pieces, but not written instructions.

5. **Cultural assumptions**
   - Rules were likely considered common knowledge.
   - Writers assumed readers already knew how to play, so they didn't explain them.

#### Why literacy wasn't widespread in the Roman Empire

1. **Social stratification and class barriers**
   - Education was mainly for the elite (wealthy families, aristocrats, senators).
   - Lower classes (peasants, slaves, laborers) had little or no access to formal education.
   - Literacy was a status marker, not a universal skill.

2. **Economic constraints**
   - Education was expensive: private tutors, writing materials (papyrus, ink, stylus), and time away from work.
   - Most people couldn't afford to have children not working.
   - Education was a private expense, not publicly funded.

3. **No public education system**
   - No state-sponsored schools for the general population.
   - Education was private and family-based.
   - Some cities had informal schools, but they weren't universal or standardized.

4. **Gender inequality**
   - Girls rarely received formal education.
   - Women's education focused on domestic skills, not literacy.
   - This roughly halved the potential literate population.

5. **Oral tradition culture**
   - Roman society relied heavily on oral communication.
   - Legal proceedings, business, storytelling, and entertainment were often oral.
   - Reading/writing was less essential for daily life than in modern societies.

6. **Practical priorities**
   - Most people needed practical skills (farming, crafts, trade) more than literacy.
   - Literacy was useful for administration, law, and literature, but not for most occupations.
   - Survival and economic activity didn't require reading.

7. **Geographic and linguistic diversity**
   - The Empire spanned many regions with different languages.
   - Latin was the administrative language, but many spoke local languages.
   - Limited standardization of education across provinces.

**Estimates:**
- Literacy rates: roughly 5–15% of the population.
- Higher in cities (especially Rome), lower in rural areas.
- Concentrated among: aristocrats, merchants, administrators, military officers, and some skilled craftsmen.

#### How Roman Emperors & Governors Communicated with Non-Roman 

1. **Multilingual administration**
   - Greek as lingua franca: In the eastern provinces, Koine Greek was widely used alongside Latin. Many Roman officials were bilingual.
   - Local languages: Officials used interpreters for languages like Punic (North Africa), Aramaic (Middle East), Celtic (Gaul/Britain), and Germanic languages.
   - Bilingual inscriptions: Public decrees were often posted in both Latin and the local language.

2. **Interpreters and translators**
   - Professional interpreters were used for legal proceedings, administrative meetings, and official communications.
   - Local bilingual elites often served as intermediaries, bridging Roman administration and local communities.

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

6. **Visual communication**
   - **Coinage**: Coins with imperial portraits and symbols circulated widely, conveying authority visually.
   - **Statues and monuments**: Imperial imagery reinforced Roman presence.
   - **Standards and symbols**: Military standards and symbols were universally recognizable.

7. **Provincial administration structure**
   - Governors had staffs that included local advisors and bilingual clerks.
   - Provincial capitals served as communication hubs where messages were translated and distributed.
   - Local councils and assemblies helped relay information to communities.

8. **Military presence**
   - The Roman army included soldiers from across the empire who could communicate in local languages.
   - Military camps served as administrative centers and communication nodes.
   - Veterans often settled in provinces and acted as cultural bridges.

**The practical reality:**
- Elite communication: Direct communication with non-Roman subjects was often indirect, through local intermediaries.
- Hierarchical transmission: Messages flowed: Emperor → Governor → Local elites → Common people.
- Pragmatic multilingualism: The administration adapted to local languages rather than forcing Latin on everyone.

This flexible, multilingual approach helped the empire govern diverse populations across vast territories, even with limited literacy.

### Legacy

Latrones influenced the development of later European strategy games and is considered a precursor to games like:
- Medieval Tafl games.
- Modern chess (in terms of strategic).
- Draughts/checkers (in terms of jumping mechanics).

<br>

# Structure

```
.
├── Cargo.toml          # Rust project configuration
├── Cargo.lock          # Dependency lock file
├── src/
│   └── lib.rs          # Main game logic (Rust/WASM)
├── pkg/                # Generated WebAssembly package (created by wasm-pack)
│   ├── latrones.js     # JavaScript bindings
│   ├── latrones_bg.wasm # WebAssembly binary
│   └── *.d.ts          # TypeScript definitions
├── index.html          # Web interface
├── style.css           # Styling
├── index.js            # JavaScript bindings and UI logic
└── README.md           # This file
```
