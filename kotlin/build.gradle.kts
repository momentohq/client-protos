plugins {
    id("com.google.protobuf") version "0.9.4" apply false
    kotlin("jvm") version "1.9.10" apply false
    id("io.github.gradle-nexus.publish-plugin") version "1.3.0"
}

ext["grpcVersion"] = "1.57.2"
ext["grpcKotlinVersion"] = "1.4.0"
ext["protobufVersion"] = "3.24.1"
ext["coroutinesVersion"] = "1.7.3"

subprojects {
    repositories {
        mavenCentral()
        google()
    }
}

nexusPublishing {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://s01.oss.sonatype.org/service/local/"))
            snapshotRepositoryUrl.set(uri("https://s01.oss.sonatype.org/content/repositories/snapshots/"))
            username.set(System.getenv("SONATYPE_USERNAME"))
            password.set(System.getenv("SONATYPE_PASSWORD"))
        }
    }
}
