plugins {
    // Auto-provision the JDK 11 toolchain (jvmToolchain(11)) when the Gradle
    // daemon runs on a newer JDK.
    id("org.gradle.toolchains.foojay-resolver-convention") version "1.0.0"
}

rootProject.name = "pragmastat"
include("demo")
