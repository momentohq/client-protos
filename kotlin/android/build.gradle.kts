plugins {
    kotlin("jvm")
    id("com.google.protobuf")
    id("maven-publish")
    signing
}

version = rootProject.version

dependencies {
    // Descriptor.proto is included because there is no lite version of common protos:
    // https://github.com/protocolbuffers/protobuf/issues/1889
    protobuf(files("./google-protos"))
    protobuf(files(fileTree("../../proto") {
        include("*.proto")
        // webhook is temporarily excluded because the kotlin code generation creates non-compiling code
        // in the case of a proto with a package that shares a name with a message field.
        exclude("webhook.proto")
        exclude("**/*/*.proto")
    }))

    api("org.jetbrains.kotlinx:kotlinx-coroutines-core:${rootProject.ext["coroutinesVersion"]}")

    api("io.grpc:grpc-stub:${rootProject.ext["grpcVersion"]}")
    api("io.grpc:grpc-protobuf-lite:${rootProject.ext["grpcVersion"]}")
    api("io.grpc:grpc-kotlin-stub:${rootProject.ext["grpcKotlinVersion"]}")
    api("com.google.protobuf:protobuf-kotlin-lite:${rootProject.ext["protobufVersion"]}")
}

java {
    withSourcesJar()
}

kotlin {
    jvmToolchain(8)
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile>().all {
    kotlinOptions {
        freeCompilerArgs = listOf("-opt-in=kotlin.RequiresOptIn")
    }
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:${rootProject.ext["protobufVersion"]}"
    }
    plugins {
        create("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:${rootProject.ext["grpcVersion"]}"
        }
        create("grpckt") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:${rootProject.ext["grpcKotlinVersion"]}:jdk8@jar"
        }
    }
    generateProtoTasks {
        all().forEach {
            it.builtins {
                named("java") {
                    option("lite")
                }
                create("kotlin") {
                    option("lite")
                }
            }
            it.plugins {
                create("grpc") {
                    option("lite")
                }
                create("grpckt") {
                    option("lite")
                }
            }
        }
    }
}

val javadocJar = tasks.register("javadocJar", Jar::class.java) {
    archiveClassifier.set("javadoc")
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
            artifactId = rootProject.name + "-android"

            artifact(javadocJar)

            pom {
                name.set("Momento Kotlin Android Client Protocols")
                description.set("Kotlin protobuf protocols for Android that define the Momento gRPC wire format")
                url.set("https://github.com/momentohq/client-protos")
                licenses {
                    license {
                        name.set("The Apache License, Version 2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                    }
                }
                developers {
                    developer {
                        id.set("momento")
                        name.set("Momento")
                        organization.set("Momento")
                        email.set("eng-deveco@momentohq.com")
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

val signingKey: String? = System.getenv("SONATYPE_SIGNING_KEY")
if (signingKey != null) {
    signing {
        useInMemoryPgpKeys(signingKey, System.getenv("SONATYPE_SIGNING_KEY_PASSWORD"))
        sign(publishing.publications["maven"])
    }
}
