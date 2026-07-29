plugins {
    kotlin("jvm") version "2.4.10"
    `maven-publish`
    signing
    id("org.jetbrains.dokka") version "2.2.0"
    id("org.jreleaser") version "1.25.0"
    id("org.jlleitschuh.gradle.ktlint") version "14.2.0"
}

group = "dev.pragmastat"
version = "14.0.1"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.14.4")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.22.1")
    testImplementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.22.1")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

testing {
    suites {
        val test by getting(JvmTestSuite::class) {
            useJUnitJupiter("5.10.1")
        }
    }
}

// The reference fixtures live outside this project, in the repository-wide tests/ directory, and
// the suites read them at runtime. Gradle keys its up-to-date check on declared inputs, so
// without this an edited fixture leaves the previous PASSED result in place and `gradlew test`
// reports success in under a second without running anything. Verified by truncating a fixture:
// the run stayed green until these lines existed.
//
// This is the same defect the Go port had, where the answer was -count=1. Declaring the input is
// the better shape here: the cache keeps working and starts telling the truth.
tasks.withType<Test>().configureEach {
    inputs
        .dir(rootProject.layout.projectDirectory.dir("../tests"))
        .withPropertyName("referenceFixtures")
        .withPathSensitivity(PathSensitivity.RELATIVE)
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
