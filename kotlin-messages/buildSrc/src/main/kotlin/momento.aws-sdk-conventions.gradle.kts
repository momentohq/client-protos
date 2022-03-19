plugins {
    id("org.jetbrains.kotlin.jvm")
}

dependencies {
    // This is just intended to pin the AWS SDk version across modules.  There are other options for how to handle this
    // but just doing this for now.
    implementation(platform("software.amazon.awssdk:bom:2.17.18"))
}
