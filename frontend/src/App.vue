<script setup>
    import {ref, onMounted} from 'vue'

    const apiData = ref("Loading...")
    const error = ref(null)

    onMounted(async () => {
        try {
            const response = await fetch("http://127.0.0.1:3000/api/home")
            if (!response.ok) throw new Error("Not ok :(")

            const data = await response.json()
            apiData.value = data.message
        } catch(err) {
            errorMsg.value = "Could not connect to backend"
            console.error(err)
        }
    })
</script>

<template>
    <main style="text-align: center; font-family: sans-serif; margin-top: 50px;">
        <h1> Please work :) </h1>
        
        <div v-if="errorMsg" style="color: red;">
            {{ errorMsg }}
        </div>
        <div v-else style="font-size: 1.2rem; color: #42b883; font-weight: bold;">
            Backend says: {{ apiData }}
        </div>
    </main>
</template>
