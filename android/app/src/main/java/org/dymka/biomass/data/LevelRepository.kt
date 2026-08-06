package org.dymka.biomass.data

data class LevelInfo(
    val id: Int,
    val title: String,
    val gridDimensions: String,
    val wallsPerTurn: Int
)

class LevelRepository {
    fun getCampaignLevels(): List<LevelInfo> {
        return listOf(
            LevelInfo(1, "Containment 101", "4x4", 2),
            LevelInfo(2, "Twin Spores", "5x5", 2),
            LevelInfo(3, "Divided Sectors", "6x6", 2),
            LevelInfo(4, "Rapid Mutation", "6x6", 2),
            LevelInfo(5, "Corridor Siege", "7x7", 3),
            LevelInfo(6, "Infection Wave", "8x8", 3),
            LevelInfo(7, "Bio-Reactor Breach", "8x8", 3),
            LevelInfo(8, "Outbreak Zero", "10x10", 4)
        )
    }
}
