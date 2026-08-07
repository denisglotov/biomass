package org.dymka.biomass.ui

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import org.dymka.biomass.BiomassApplication
import org.dymka.biomass.viewmodel.BiomassViewModel

class MainActivity : ComponentActivity() {

    private val viewModel: BiomassViewModel by viewModels {
        object : ViewModelProvider.Factory {
            @Suppress("UNCHECKED_CAST")
            override fun <T : ViewModel> create(modelClass: Class<T>): T {
                val app = application as BiomassApplication
                return BiomassViewModel(app.container.levelRepository) as T
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            BiomassGameScreen(viewModel = viewModel)
        }
    }
}
