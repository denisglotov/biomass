package org.dymka.biomass.di

import org.dymka.biomass.data.LevelRepository

/**
 * Manual Dependency Injection Container for Biomass Android app.
 * Avoids heavy Dagger/Hilt annotation-processing boilerplate while maintaining clean architecture.
 */
class AppContainer {
    val levelRepository: LevelRepository by lazy {
        LevelRepository()
    }
}
