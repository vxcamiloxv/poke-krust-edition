package org.pokekotlin

import com.sun.org.apache.xalan.internal.xsltc.compiler.util.Type.Text
import io.ktor.application.call
import io.ktor.http.ContentType
import io.ktor.request.receive
import io.ktor.response.respondText
import io.ktor.routing.Routing
import io.ktor.routing.get
import io.ktor.routing.post


private val water_pokemon = "{\"pokemon\":{\"water\":[\"Squirtle\",\"Vaporeon\",\"Milotic\",\"Kyogre\",\"Tentacool\"]}}"


    fun Routing.rootGet() {

        get("/") {
            call.respondText("Pokemon with Kotlin + RUST", contentType = ContentType.Text.Plain)
        }

    }


    fun Routing.rootPost() {
        post("/") {
            val post = call.receive<String>()
            call.respondText("Post request $post", ContentType.Text.Plain)
        }
    }


    fun Routing.rootGetWaterPokemon() {
        get("/waterPokemon") {
            call.respondText(water_pokemon)
        }
    }
