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

- `PlayerSpawn`, you can add a `facing_dir` field (of type `Float`) to specify the spawn direction (`-1.0` for left, `1.0` for right)

Without `PlayerSpawn`, no player will be created.

### Collision

In `Collisions`, the IntGrid value `1` is interpreted as a solid wall.

### Game Registery

To make the world visible in the game's selection menu, you have to add it into the Game Registery.
It is located in `src/system/resources.rs` (under `impl Default for GameRegistry`). Here, you can add a new entry to the `worlds` vector with the path to your `.ldtk` file and a display name.

## 3. Recommended LDtk Workflow

1. Duplicate an existing level (e.g. `cave.ldtk`) and rename it.
2. Keep the same layer and entity definitions.
3. Draw the level art in `VisualBackground` and `VisualForeground`.
4. Paint collisions in `Collisions` using the value `1`.
5. Place one `PlayerSpawn` in the `Entities` layer.
6. Edit the player spawn direction if needed by adding a `facing_dir` field to the entity and setting it to `-1.0` for left or `1.0` for right.
7. Save the LDtk project.
8. Add the new level to the Game Registery in `src/system/resources.rs` to make it visible in the game's selection menu.
9. Test the level by running the game and selecting it from the menu.

## 5. Quick Pre-Test Checklist

- [ ] All 4 layers exist with the correct names.
- [ ] `Collisions` contains `1` cells wherever the player should be able to stand.
- [ ] One `PlayerSpawn` is present.
- [ ] The grid is set to `8 px`.
- [ ] The level is added to the Game Registery.

## 6. Troubleshooting

### Blank Screen or Broken Rendering

- Check layer names. They are case-sensitive.
- Check that the tilesets referenced by LDtk still exist.

### Player Does Not Spawn

- Check that the `PlayerSpawn` entity exists in `Entities`.

### Player Falls Through the Level

- Check that the `Collisions` layer uses the IntGrid type.
- Check that wall tiles use the value `1`.
