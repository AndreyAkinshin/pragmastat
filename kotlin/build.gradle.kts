plugins {
    kotlin("jvm") version "1.9.22"
    `maven-publish`
    signing
    id("org.jetbrains.dokka") version "1.9.20"
    id("org.jreleaser") version "1.15.0"
}

group = "dev.pragmastat"
version = "3.1.14"

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

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
            artifact(sourcesJar)
            artifact(javadocJar)

            pom {
                name.set("Pragmastat")
                description.set("Pragmastat: Pragmatic Statistical Toolkit")
                url.set("https://pragmastat.dev")

                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }

                developers {
                    developer {
                        id.set("AndreyAkinshin")
                        name.set("Andrey Akinshin")
                        email.set("andrey.akinshin@gmail.com")
                    }
                }

                scm {
                    connection.set("scm:git:https://github.com/AndreyAkinshin/pragmastat.git")
                    developerConnection.set("scm:git:ssh://git@github.com/AndreyAkinshin/pragmastat.git")
                    url.set("https://github.com/AndreyAkinshin/pragmastat")
                }

                properties.set(mapOf(
                    "doi" to "10.5281/zenodo.17236778",
                    "keywords" to "statistics"
                ))
            }
        }
    }
}

signing {
    useInMemoryPgpKeys(System.getenv("GRADLE_SIGNING_KEY"), System.getenv("GRADLE_SIGNING_PASSWORD"))
    sign(publishing.publications)
}

jreleaser {
    project {
        authors.set(listOf("Andrey Akinshin"))
        license.set("MIT")
        description.set("Pragmastat: Pragmatic Statistical Toolkit")
        inceptionYear.set("2024")
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
