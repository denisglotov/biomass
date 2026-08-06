package org.dymka.biomass.ui

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import org.dymka.biomass.di.AppContainer
import org.dymka.biomass.viewmodel.BiomassViewModel

class MainActivity : ComponentActivity() {

    // AppContainer initialized for Manual Dependency Injection
    private lateinit var appContainer: AppContainer
    private lateinit var viewModel: BiomassViewModel

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Manual Dependency Injection setup
        appContainer = AppContainer()
        viewModel = BiomassViewModel(appContainer.levelRepository)

        setContent {
            BiomassGameScreen(viewModel = viewModel)
        }
    }
}
