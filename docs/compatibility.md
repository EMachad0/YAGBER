# YAGBER Game Compatibility

Taking a look at the existing CGB games on the [Game Boy Hardware Database](https://gbhwdb.gekkio.fi/cartridges/gbc.html) this emulator should be capable to run over 90% of the oficially released games.

This is because YAGBER supports cartridges without MBCs(Memory Bank Controllers) or with MBCs 1, 2, 3 or 5. This account to the vast majority of game boy color games but there are other exotic games that use other MBCs types and thus are not yet supported.

### Why should?

Because not all games were manually tested but here is a few that are known to be compatible/incompatible:

|Game|Image|Compatibility|Notes|
|----|-----|-------------|-----|
|Tetris|![TetrisImg](https://upload.wikimedia.org/wikipedia/en/4/4a/Tetris_Boxshot.jpg)|:check:|No ram, Fully compatible as all original game boy games should be|
|F1Race|![F1RaceImg](https://m.media-amazon.com/images/I/61NRBKWungL._UF1000,1000_QL80_.jpg)|:check:|MBC2 game with battery backed ram allowing saving and loading|
|Super Mario Land 2 - 6 Golden Coins|![SuperMarioLand2Img](https://upload.wikimedia.org/wikipedia/en/0/0d/Super_Mario_Land_2_box_art.jpg)|:check:||
|Pokemon - Gold Version|![PokemonGoldImg](https://i5.walmartimages.com/asr/f9386c4c-07b1-4745-ada8-dc353bcf669b.a829294791b071d3f5fc58fff3396c8f.jpeg)|:check:|MBC3 game with real time clock|
|Pokemon - Yellow Version|![PokemonYellowImg](https://upload.wikimedia.org/wikipedia/pt/4/43/Pok%C3%A9mon_Yellow_cover.png)|:check:|MBC5 game|
|Pocket Monsters - Crystal Version|![PokemonCrystal](https://upload.wikimedia.org/wikipedia/en/8/84/Pok%C3%A9mon_Crystal_box_art.png)|:x:|Requires MBC3 with 64Kbs of SRAM|
|Yu-Gi-Oh! - Dark Duel Stories|![YuGiOh_Dark_duel_Stories_cover](https://upload.wikimedia.org/wikipedia/en/b/bd/Yu-Gi-Oh_Dark_duel_Stories_cover.jpg)|:warning:|Playable with small visual/audio artifacts|

