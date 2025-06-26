plugins {
    kotlin("jvm") version "1.9.22"
    `maven-publish`
    application
}

application {
    mainClass.set("com.pragmastat.example.MainKt")
}

group = "com.pragmastat"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.1")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.16.1")
    testImplementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.16.1")
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(11)
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
            
            pom {
                name.set("Pragmastat")
                description.set("Pragmastat: Unified Statistical Toolkit for Reliable Analysis of Real-World Data")
                url.set("https://github.com/AndreyAkinshin/pragmastat")
                
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
                    connection.set("scm:git:git://github.com/AndreyAkinshin/pragmastat.git")
                    developerConnection.set("scm:git:ssh://github.com/AndreyAkinshin/pragmastat.git")
                    url.set("https://github.com/AndreyAkinshin/pragmastat")
                }
            }
        }
    }
}