plugins {
    kotlin("jvm")
    id("com.google.protobuf")
    id("maven-publish")
    signing
}

version = rootProject.version

dependencies {
    // The exclude stops the google protos from being generated.
    // The dependencies include prebuilt versions.
    protobuf(files(fileTree("../../proto") {
        include("*.proto")
        exclude("**/*/*.proto")
    }))

    api("org.jetbrains.kotlinx:kotlinx-coroutines-core:${rootProject.ext["coroutinesVersion"]}")

    api("io.grpc:grpc-stub:${rootProject.ext["grpcVersion"]}")
    api("io.grpc:grpc-protobuf:${rootProject.ext["grpcVersion"]}")
    api("io.grpc:grpc-kotlin-stub:${rootProject.ext["grpcKotlinVersion"]}")
    api("com.google.protobuf:protobuf-kotlin:${rootProject.ext["protobufVersion"]}")
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
                named("java")
            }
            it.plugins {
                create("grpc")
                create("grpckt")
            }
            it.generateDescriptorSet = true
            it.descriptorSetOptions.path = "$projectDir/generated-sources/descriptors/client_protos.dsc"
            it.descriptorSetOptions.includeImports = true
            it.descriptorSetOptions.includeSourceInfo = true
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
            artifactId = rootProject.name + "-jvm"

            artifact(javadocJar)

            pom {
                name.set("Momento Kotlin JVM Client Protocols")
                description.set("Kotlin protobuf protocols for the JVM that define the Momento gRPC wire format")
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
