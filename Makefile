.PHONY: setup build build-android install-android start clean lint

BOB_URL := https://d.defold.com/archive/f735c12192bf95684e6ae1ae27c400b8170fc6d8/bob/bob.jar
JAVA_HOME_PATH := /Library/Java/JavaVirtualMachines/temurin-26.jdk/Contents/Home
JAVA := $(JAVA_HOME_PATH)/bin/java

setup:
	curl -L -o bob.jar $(BOB_URL)

build:
	JAVA_HOME=$(JAVA_HOME_PATH) $(JAVA) -jar bob.jar --archive --platform wasm-web --architectures wasm-web --use-uncompressed-lua-source distclean build bundle --bundle-output dist

build-android:
	JAVA_HOME=$(JAVA_HOME_PATH) $(JAVA) -jar bob.jar --archive --platform armv7-android --architectures armv7-android,arm64-android --use-uncompressed-lua-source distclean build bundle --bundle-output dist-android

install-android:
	adb $(if $(DEVICE),-s $(DEVICE)) install -r dist-android/Biomass/Biomass.apk

start:
	python3 -m http.server 3000 -d dist/Biomass

lint:
	luacheck .

clean:
	rm -rf dist/ dist-android/
