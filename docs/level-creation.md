# Level Creation Guide (LDtk)

This guide explains how to create a playable level for OpenTFA with LDtk.

## 1. Prerequisites

- It is recommended to use latest version of [LDtk](https://ldtk.io/)
- Levels are located at: `assets/levels/`.
- The grid size expected by the code is `8 px`.

## 2. Structure Expected by the Game

The Rust runtime depends on exact names defined in LDtk.

### Required Layers

Layer order does not matter as long as their Z position is derived from their name. However, the following order is recommended for better readability in LDtk. The following 4 layers are required:

- `VisualForeground` (Tiles)
- `Entities` (Entities)
- `VisualBackground` (Tiles)
- `Collisions` (IntGrid)

These identifiers are used in the code for rendering order and loading.
If an extra layer is present, a warning will be displayed and its position cannot be guaranteed.

> [!NOTE]
> `IntGrid` layers are **not** rendered.

### Required Entities

- `PlayerSpawn`

Without `PlayerSpawn`, no player will be created.

### Collision

In `Collisions`, the IntGrid value `1` is interpreted as a solid wall.

## 3. Recommended LDtk Workflow

1. Duplicate an existing level (e.g. `cave.ldtk`) and rename it.
2. Keep the same layer and entity definitions.
3. Draw the level art in `VisualBackground` and `VisualForeground`.
4. Paint collisions in `Collisions` using the value `1`.
5. Place one `PlayerSpawn` in the `Entities` layer.
6. Save the LDtk project.

## 4. Integrating a New Level Into the Game

Current code behavior:

- The loaded world is hardcoded to `assets/levels/cave.ldtk`.
- The loaded level is `LevelSelection::index(0)`.

So, to test a new level quickly, the simplest options are:

- replace the contents of `assets/levels/cave.ldtk`
- add your level to the same project while keeping index `0` for the level you want to test
- replace the hardcoded path and index in the code, then recompile

## 5. Quick Pre-Test Checklist

- [ ] All 4 layers exist with the correct names.
- [ ] `Collisions` contains `1` cells wherever the player should be able to stand.
- [ ] One `PlayerSpawn` is present.
- [ ] The grid is set to `8 px`.

## 6. Troubleshooting

### Blank Screen or Broken Rendering

- Check layer names. They are case-sensitive.
- Check that the tilesets referenced by LDtk still exist.

### Player Does Not Spawn

- Check that the `PlayerSpawn` entity exists in `Entities`.

### Player Falls Through the Level

- Check that the `Collisions` layer uses the IntGrid type.
- Check that wall tiles use the value `1`.

## 7. Recommended Next Improvements

For a simpler level pipeline, the next useful improvements would be:

- dynamic `.ldtk` file selection instead of hardcoding `cave.ldtk`,
- level selection by LDtk identifier instead of index `0`,
- automatic validation that fails on startup when required layers or entities are missing.
