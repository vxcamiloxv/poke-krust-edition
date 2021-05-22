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


@Suppress("unused")
@kotlin.jvm.JvmOverloads
fun Application.module(testing: Boolean = false) {
    routing {

        this.rootGet()
        this.rootPost()
        this.rootGetWaterPokemon()

    }
}

fun main(args: Array<String>): Unit = io.ktor.server.netty.EngineMain.main(args)


