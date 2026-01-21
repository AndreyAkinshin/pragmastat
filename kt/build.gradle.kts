plugins {
    kotlin("jvm") version "2.0.21"
    application
    `maven-publish`
    signing
    id("org.jetbrains.dokka") version "2.0.0"
    id("org.jreleaser") version "1.20.0"
}

group = "dev.pragmastat"
version = "4.0.1"

repositories {
    mavenCentral()
}

application {
    mainClass.set("dev.pragmastat.demo.MainKt")
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

dokka {
    dokkaPublications.html {
        outputDirectory.set(layout.buildDirectory.dir("dokka/html"))
    }
}

val sourcesJar by tasks.registering(Jar::class) {
    archiveClassifier.set("sources")
    from(sourceSets["main"].allSource)
}

val javadocJar by tasks.registering(Jar::class) {
    archiveClassifier.set("javadoc")
    dependsOn(tasks.dokkaGeneratePublicationHtml)
    from(tasks.dokkaGeneratePublicationHtml.flatMap { it.outputDirectory })
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
            artifact(sourcesJar)
            artifact(javadocJar)

            pom {
                name.set("pragmastat")
                description.set("Pragmastat: Pragmatic Statistical Toolkit")
                url.set("https://pragmastat.dev")
                inceptionYear.set("2025")
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }
                developers {
                    developer {
                        id.set("akinshin")
                        name.set("Andrey Akinshin")
                    }
                }
                scm {
                    connection.set("scm:git:git://github.com/AndreyAkinshin/pragmastat.git")
                    developerConnection.set("scm:git:ssh://github.com/AndreyAkinshin/pragmastat.git")
                    url.set("https://github.com/AndreyAkinshin/pragmastat")
                }
            }
        }
    }
    repositories {
        maven {
            name = "staging"
            url = uri(layout.buildDirectory.dir("staging-deploy"))
        }
    }
}

signing {
    val signingKey = System.getenv("GRADLE_SIGNING_KEY")
    val signingPassword = System.getenv("GRADLE_SIGNING_PASSWORD")
    if (signingKey != null && signingPassword != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
        sign(publishing.publications["maven"])
    }
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
                }
            }
        }
    }
}
