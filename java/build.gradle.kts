import com.google.protobuf.gradle.generateProtoTasks
import com.google.protobuf.gradle.id
import com.google.protobuf.gradle.plugins
import com.google.protobuf.gradle.protobuf
import com.google.protobuf.gradle.protoc

plugins {
    id("momento.java-library-conventions")

    id("com.google.protobuf") version "0.8.16"
    idea
    id("io.github.gradle-nexus.publish-plugin") version "1.3.0"
    id("maven-publish")
    id("signing")
    id("ca.cutterslade.analyze") version "1.9.0"
}

// Use a default SNAPSHOT version if the environment variable cannot be found.
// The version is specified here to prevent an inconsistent version from being seen by different tasks.
version = System.getenv("JAVA_PROTOS_VERSION") ?: "0.1.0-SNAPSHOT"

val grpcProtobufVersion = "3.22.2"
val grpcVersion = "1.53.0"

dependencies {
    api(platform("io.grpc:grpc-bom:$grpcVersion"))
    api(platform("com.google.protobuf:protobuf-bom:$grpcProtobufVersion"))

    api("io.grpc:grpc-stub")
    api("io.grpc:grpc-api")
    api("com.google.protobuf:protobuf-java")
    api("com.google.guava:guava:31.1-android") // version pulled from protobuf-java

    implementation("io.grpc:grpc-protobuf")
    implementation("com.google.api.grpc:proto-google-common-protos:2.9.0") // version pulled from grpc-protobuf

    compileOnly("javax.annotation:javax.annotation-api:1.3.2")

    protobuf(files("../proto/"))
}

// Required to generate source and javadoc jars with anything in them
// Will no longer be needed with protobuf plugin version 9+
sourceSets.main {
    java.srcDirs(
            "build/generated/source/proto/main/grpc",
            "build/generated/source/proto/main/java",
    )
}

java {
    withJavadocJar()
    withSourcesJar()
    // Produce an artifact build for Java 8+
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(8))
    }
}

protobuf {
    var systemOverride = ""
    if (System.getProperty("os.name") == "Mac OS X") {
        println("overiding protobuf artifacts classifier to osx-x86_64 so M1 Macs can find lib")
        systemOverride = ":osx-x86_64"
    }
    protoc {
        artifact = "com.google.protobuf:protoc:$grpcProtobufVersion$systemOverride"
    }
    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:$grpcVersion$systemOverride"
        }
    }
    generateProtoTasks {
        all().forEach { task ->
            task.plugins {
                id("grpc")
            }
            task.generateDescriptorSet = true
            task.descriptorSetOptions.path = "$projectDir/generated-sources/descriptors/client_protos.dsc"
            task.descriptorSetOptions.includeImports = true
            task.descriptorSetOptions.includeSourceInfo = true
        }
    }
}

// Only run publishing tasks if the required environment variables are present
val safeToPublish = provider {
    !System.getenv("JAVA_PROTOS_VERSION").isNullOrEmpty() &&
            !System.getenv("SONATYPE_USERNAME").isNullOrEmpty() &&
            !System.getenv("SONATYPE_PASSWORD").isNullOrEmpty() &&
            !System.getenv("SONATYPE_SIGNING_KEY").isNullOrEmpty() &&
            !System.getenv("SONATYPE_SIGNING_KEY_PASSWORD").isNullOrEmpty()
}

tasks.withType<AbstractPublishToMaven>().configureEach {
    onlyIf {
        safeToPublish.get()
    }
}
publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            artifactId = rootProject.name

            pom {
                name.set("Momento Client Protocols")
                description.set("Java protobuf protocols that define the Momento gRPC wire format")
                url.set("https://github.com/momentohq/client-protos")
                licenses {
                    license {
                        name.set("The Apache License, Version 2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                    }
                }
                developers {
                    developer {
                        id.set("nand4011")
                        name.set("Nate Anderson")
                        organization.set("Momento")
                    }
                }
                scm {
                    connection.set("scm:git:git://github.com/momentohq/client-protos.git")
                    developerConnection.set("scm:git:git@github.com:momentohq/client-protos.git")
                    url.set("https://github.com/momentohq/client-protos")
                }
            }
        }
    }
}

tasks.withType<io.github.gradlenexus.publishplugin.AbstractNexusStagingRepositoryTask>().configureEach {
    onlyIf {
        safeToPublish.get()
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

tasks.withType<Sign>().configureEach {
    onlyIf {
        safeToPublish.get()
    }
}
signing {
    val signingKey = System.getenv("SONATYPE_SIGNING_KEY")
    val signingPassword = System.getenv("SONATYPE_SIGNING_KEY_PASSWORD")

    useInMemoryPgpKeys(signingKey, signingPassword)
    sign(publishing.publications["mavenJava"])
}
