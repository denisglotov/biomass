package org.dymka.biomass

import android.app.Application
import org.dymka.biomass.di.AppContainer

class BiomassApplication : Application() {
    lateinit var container: AppContainer

    override fun onCreate() {
        super.onCreate()
        container = AppContainer()
    }
}
