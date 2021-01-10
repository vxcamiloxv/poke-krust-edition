package org.pokekotlin

import io.ktor.application.*
import io.ktor.http.*
import io.ktor.request.*
import io.ktor.response.*
import io.ktor.routing.*

    private val water_pokemon = "{\"pokemon\":{\"water\":[\"Squirtle\",\"Vaporeon\",\"Milotic\",\"Kyogre\",\"Tentacool\"]}}"


    fun Routing.rootGet() {

        get("/") {
            call.respondText("Pokemon with Kotlin + RUST", contentType = ContentType.Text.Plain)
        }

    }


    fun Routing.rootPost(){
        post("/") {
            val post = call.receive<String>()
            call.respondText("Post request $post", ContentType.Text.Plain)
        }
    }


    fun Routing.rootGetWaterPokemon(){
        get("/waterPokemon"){
            call.respondText(water_pokemon)
        }
    }