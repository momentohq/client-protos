/***********************************************************************************************************************
 * Re-usable code for fetching and caching CodeArtifact credentials.
 *
 * This file is a bit unfortunate; it's got quite a bit of code in it that we will need to re-use across all of our
 * projects that consume artifacts from CodeArtifact.  However, there is a chicken-and-egg problem; since this code is
 * what gives us read access to the CodeArtifact repos, we can't put this into a gradle plugin as a jar that we store
 * in CodeArtifact, because we need the creds before we can read from the repo.  Therefore we'll likely have to copy
 * and paste this file across the git repos, unfortunately.  Open to better ideas!
 *
 * The way this works is:
 * - assumes you have configured a `dev` profile in `~/aws/config`, and that that profile has read access to our codeartifact
 *   repo.
 * - Makes an API call to codeartifact to get a token; caches creds in ~/.momento/cache.  Re-uses existing creds from
 *   the cache file if it is present and if they are not expired.
 * - Supports an env var CODEARTIFACT_AUTH_TOKEN, which, if set, will supersede all of the above behavior.
 **********************************************************************************************************************/
object CodeArtifactAuth {
    data class CodeArtifactCreds(val authorizationToken: String, val expiration: java.time.Instant)

    private val CODEARTIFACT_CREDS_CACHE_FILE = File(System.getProperty("user.home"))
        .resolve(".momento/cache/codeartifact-token.properties")

    private var codeArtifactToken: String? = null

    fun getToken(): String {
        if (codeArtifactToken == null) {
            val envVar = System.getenv("CODEARTIFACT_AUTH_TOKEN")
            if (envVar != null) {
                codeArtifactToken = envVar
            } else {
                codeArtifactToken = getCodeArtifactTokenFromCache()
            }
        }
        return codeArtifactToken!!
    }

    private fun getCodeArtifactTokenFromCache(): String {
        println("Checking for existence of code artifact creds cache file: $CODEARTIFACT_CREDS_CACHE_FILE")
        if (! CODEARTIFACT_CREDS_CACHE_FILE.exists()) {

            println("Cache file does not exist, creating it.")
            refreshCacheFile()
        }
        var creds = loadCredsFromCacheFile()
        if (creds.expiration.isBefore(java.time.Instant.now())) {
            println("CodeArtifact credentials expired, refreshing.")
            refreshCacheFile()
            creds = loadCredsFromCacheFile()
        }
        return creds.authorizationToken
    }

    private fun refreshCacheFile() {
        CODEARTIFACT_CREDS_CACHE_FILE.parentFile.mkdirs()
        val creds = runCodeArtifactTokenCommand()
        creds.store(CODEARTIFACT_CREDS_CACHE_FILE.outputStream(), null)
    }

    private fun loadCredsFromCacheFile(): CodeArtifactCreds {
        val creds = java.util.Properties().apply { load(CODEARTIFACT_CREDS_CACHE_FILE.inputStream()) }
        val authorizationToken = creds.getProperty("authorizationToken")
//      if there are issues with formatting the datetime (Text '2021-09-17T11:38:59+00:00' could not be parsed at index 19)
//      try java.time.format.DateTimeFormatter.ISO_DATE_TIME
        val expiration = java.time.Instant.from(java.time.format.DateTimeFormatter.ISO_INSTANT.parse(creds.getProperty("expiration")))
        return CodeArtifactCreds(
            authorizationToken,
            expiration
        )
    }

    private fun runCodeArtifactTokenCommand(): java.util.Properties {
        println("Making call to code artifact API to get token profile=dev")
        val commandOutput = runCommand(
            File("."),
            listOf(
                "aws",
                "codeartifact",
                "get-authorization-token",
                "--domain",
                "momento-prod",
                "--domain-owner",
                "401011790710",
                "--output",
                "text",
                "--region",
                "us-west-2",
                "--profile",
                "dev"
            )
        )
        if (commandOutput != null) {
            if (commandOutput.trim() != "") {
                println("Successfully acquired CodeArtifact token")
                val fields = commandOutput.trim().split("\t")
                return java.util.Properties().apply {
                    put("authorizationToken", fields[0])
                    put("expiration", fields[1])
                }
            }
        }
        throw IllegalStateException("Unable to get token via CLI command output: ${commandOutput}")
    }

    private fun runCommand(workingDir: File, command: List<String>): String? {
        try {
            val pb = ProcessBuilder(*command.toTypedArray())
                .directory(workingDir)
                .redirectOutput(ProcessBuilder.Redirect.PIPE)
                .redirectError(ProcessBuilder.Redirect.PIPE)

            val proc = pb.start()

            proc.waitFor(1, TimeUnit.MINUTES)
            val stderr = proc.errorStream.bufferedReader().readText().trim()
            if (stderr != "") {
                println("\n\nWARNING:\n")
                println(stderr)
                println("\n\n")
            }
            return proc.inputStream.bufferedReader().readText()
        } catch (e: java.io.IOException) {
            e.printStackTrace()
            return null
        }
    }
}

project.extra["getCodeArtifactToken"] = { CodeArtifactAuth.getToken() }
