-- .luacheckrc configuration for Biomass Defold project
std = "lua51"

-- Defold Engine & Lua Globals
globals = {
    "msg",
    "hash",
    "sound",
    "go",
    "factory",
    "collectionfactory",
    "resource",
    "sys",
    "vmath",
    "gui",
    "render",
    "tilemap",
    "particlefx",
    "spine",
    "model",
    "physics",
    "zlib",
    "init",
    "final",
    "update",
    "fixed_update",
    "on_message",
    "on_input",
    "on_reload",
}

-- Ignore whitespace-only lines
ignore = {
    "631", -- line is too long
}
