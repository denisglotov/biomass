package org.dymka.biomass.ui

import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.AndroidView

@Composable
fun BiomassGameScreen() {
    Box(
        modifier = Modifier.fillMaxSize(),
    ) {
        AndroidView(
            factory = { context ->
                WebView(context).apply {
                    settings.apply {
                        javaScriptEnabled = true
                        domStorageEnabled = true
                        allowFileAccess = true
                        cacheMode = WebSettings.LOAD_NO_CACHE
                    }
                    webViewClient = WebViewClient()
                    loadUrl("file:///android_asset/index.html")
                }
            },
            modifier = Modifier.fillMaxSize(),
        )
    }
}
