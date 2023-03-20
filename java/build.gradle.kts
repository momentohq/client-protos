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

val grpcProtobufVersion = "3.21.2"
val grpcVersion = "1.47.0"
val guavaVersion = "31.1-android"

dependencies {
    implementation("io.grpc:grpc-protobuf:$grpcVersion")
    implementation("com.google.api.grpc:proto-google-common-protos:2.0.1")

    implementation("com.google.protobuf:protobuf-java:${grpcProtobufVersion}")
    implementation("com.google.guava:guava:${guavaVersion}")

    implementation("io.grpc:grpc-stub:${grpcVersion}")
    implementation("io.grpc:grpc-api:${grpcVersion}")
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

publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            version = System.getenv("JAVA_PROTOS_VERSION")
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

nexusPublishing {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://s01.oss.sonatype.org/service/local/"))
            snapshotRepositoryUrl.set(uri("https://s01.oss.sonatype.org/content/repositories/snapshots/"))
            username.set(System.getenv("OSSRH_USERNAME"))
            password.set(System.getenv("OSSRH_PASSWORD"))
        }
    }
}

signing {
    val signingKey: String = System.getenv("SIGNING_KEY")
    val signingPassword: String = System.getenv("SIGNING_PASSWORD")
    useInMemoryPgpKeys(signingKey, signingPassword)
    sign(publishing.publications["mavenJava"])
}
