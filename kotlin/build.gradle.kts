plugins {
    kotlin("jvm") version "1.9.22"
    `maven-publish`
    signing
    id("org.jetbrains.dokka") version "1.9.20"
    id("org.jreleaser") version "1.15.0"
}

group = "dev.pragmastat"
version = "3.1.18"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.1")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.16.1")
    testImplementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.16.1")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

testing {
    suites {
        val test by getting(JvmTestSuite::class) {
            useJUnitJupiter("5.10.1")
        }
    }
}

kotlin {
    jvmToolchain(11)
    sourceSets {
        main {
            kotlin.srcDir("src/main/kotlin")
        }
    }
}

val sourcesJar by tasks.registering(Jar::class) {
    archiveClassifier.set("sources")
    from(sourceSets["main"].allSource)
}

val javadocJar by tasks.registering(Jar::class) {
    archiveClassifier.set("javadoc")
    from(tasks.named("dokkaJavadoc"))
}

jreleaser {
    project {
        authors.set(listOf("Andrey Akinshin"))
        license.set("MIT")
        description.set("Pragmastat: Pragmatic Statistical Toolkit")
        inceptionYear.set("2025")
        links {
            homepage.set("https://pragmastat.dev")
        }
    }

    signing {
        active.set(org.jreleaser.model.Active.ALWAYS)
        armored.set(true)
    }

    deploy {
        maven {
            mavenCentral {
                create("sonatype") {
                    active.set(org.jreleaser.model.Active.ALWAYS)
                    url.set("https://central.sonatype.com/api/v1/publisher")
                    stagingRepository("build/staging-deploy")
                    applyMavenCentralRules.set(false)
                }
            }
        }
    }
}
