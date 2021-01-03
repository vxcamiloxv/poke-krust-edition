package org.pokekotlin

import com.typesafe.config.ConfigFactory
import io.ktor.application.*
import io.ktor.config.HoconApplicationConfig
import io.ktor.features.*
import io.ktor.http.*
import io.ktor.response.*
import io.ktor.request.*
import io.ktor.routing.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import org.slf4j.LoggerFactory

@Suppress("unused") // Referenced in application.conf
@kotlin.jvm.JvmOverloads
fun Application.module(testing: Boolean = false) {
    routing {
        get("/") {
            call.respondText("Pokemon with Kotlin + RUST")
        }
    }
}

fun main(args: Array<String>) {

    embeddedServer(Netty, port = 8080) {
        routing {
            get("/") {
                call.respondText("Pokemon with Kotlin + RUST")
            }
        }
    }.start(true)

}


