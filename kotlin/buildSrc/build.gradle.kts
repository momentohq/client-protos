apply(from = "${project.rootDir}/src/main/kotlin/momento.shared.codeartifact-token.gradle.kts")
val getCodeArtifactToken: () -> String by extra

repositories {
    maven {
        url = uri("https://momento-prod-401011790710.d.codeartifact.us-west-2.amazonaws.com/maven/maven-upstream/")
        credentials {
            username = "aws"
            password = getCodeArtifactToken()
        }
    }
}

dependencies {
    implementation(kotlin("gradle-plugin"))
    implementation("org.jlleitschuh.gradle:ktlint-gradle:10.1.0")
    implementation("gradle.plugin.com.github.jengelman.gradle.plugins:shadow:7.0.0")
    implementation(kotlin("serialization"))
}

plugins {
    kotlin("jvm") version "1.5.21"
    `kotlin-dsl`
    `idea`
}