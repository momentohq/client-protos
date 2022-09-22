import com.google.protobuf.gradle.generateProtoTasks
import com.google.protobuf.gradle.id
import com.google.protobuf.gradle.plugins
import com.google.protobuf.gradle.protobuf
import com.google.protobuf.gradle.protoc

plugins {
    id("momento.java-library-conventions")

    kotlin("jvm")
    id("com.google.protobuf") version "0.8.16"
    idea
}

val grpcProtobufVersion = "3.21.2"
val grpcVersion = "1.47.0"

dependencies {
    implementation("io.grpc:grpc-protobuf:$grpcVersion")
    implementation("io.grpc:grpc-stub:${grpcVersion}")
    implementation("javax.annotation:javax.annotation-api:1.3.2")

    protobuf(files("../proto/"))
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
