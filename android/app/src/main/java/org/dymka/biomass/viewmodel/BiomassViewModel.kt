package org.dymka.biomass.viewmodel

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import org.dymka.biomass.data.LevelInfo
import org.dymka.biomass.data.LevelRepository

class BiomassViewModel(
    private val repository: LevelRepository,
) : ViewModel() {
    private val _currentLevel = MutableStateFlow(1)
    val currentLevel: StateFlow<Int> = _currentLevel.asStateFlow()

    val levels: List<LevelInfo> = repository.getCampaignLevels()

    fun selectLevel(levelId: Int) {
        _currentLevel.value = levelId
    }
}
