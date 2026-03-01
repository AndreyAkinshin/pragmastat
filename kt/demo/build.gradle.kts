plugins {
    kotlin("jvm")
    application
}

repositories {
    mavenCentral()
}

application {
    mainClass.set("dev.pragmastat.demo.MainKt")
}

dependencies {
    implementation(rootProject)
}
