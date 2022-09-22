import com.google.protobuf.gradle.generateProtoTasks
import com.google.protobuf.gradle.id
import com.google.protobuf.gradle.plugins
import com.google.protobuf.gradle.protobuf
import com.google.protobuf.gradle.protoc

plugins {
    id("momento.kotlin-library-conventions")

    kotlin("jvm")
    id("com.google.protobuf") version "0.8.16"
    idea
}

val grpcKotlinVersion = "1.3.0"
val grpcProtobufVersion = "3.21.2"
val grpcVersion = "1.47.0"

dependencies {
    implementation(kotlin("stdlib"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.5.1")

    implementation("io.grpc:grpc-protobuf:$grpcVersion")
    implementation("com.google.protobuf:protobuf-java-util:$grpcProtobufVersion")
    implementation("io.grpc:grpc-kotlin-stub:$grpcKotlinVersion")

    protobuf(files("../../../proto/"))
}

publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            groupId = "momento.client_protos"
            artifactId = "messages"
        }
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
        id("grpckt") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:$grpcKotlinVersion:jdk8@jar"
        }
    }
    generateProtoTasks {
        all().forEach { task ->
            task.plugins {
                id("grpc")
                id("grpckt")
            }
            task.generateDescriptorSet = true
            task.descriptorSetOptions.path = "$projectDir/generated-sources/descriptors/client_protos.dsc"
            task.descriptorSetOptions.includeImports = true
            task.descriptorSetOptions.includeSourceInfo = true
        }
    }
}
