-- Audio module for Biomass
-- Handles SFX trigger calls for wall placement, biomass spread, isolation die-off, win, loss, button clicks.

local M = {}

M.sound_enabled = true

-- Proxy reference to Defold's built-in runtime sound engine module
local sound = _G["sound"]

function M.toggle_sound()
    M.sound_enabled = not M.sound_enabled
    return M.sound_enabled
end

function M.play(sound_name)
    if not M.sound_enabled then return end

    if sound and sound.play then
        pcall(function()
            sound.play("#sfx_" .. sound_name)
        end)
    end
end

return M
