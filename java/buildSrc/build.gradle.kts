
repositories {
    mavenCentral()
    gradlePluginPortal()
}


dependencies {
    implementation(kotlin("gradle-plugin"))
    implementation("ca.cutterslade.gradle:gradle-dependency-analyze:1.9.0")
}

plugins {
    kotlin("jvm") version "1.6.0"
    `kotlin-dsl`
    idea
}
