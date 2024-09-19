<script lang="ts" setup>
const number_display = ref(Math.floor(Math.random() * 100));

const { $io } = useNuxtApp() as any;
$io.connect();

// console.log("Socket URL: " + useRuntimeConfig().public.socketUrl);
// console.log("Socket Port: " + useRuntimeConfig().public.socketPort);

// Get Started Button
async function send_trigger() {
    console.log("Sending trigger");

    await $io.emit("send-trigger", "hi");
}
</script>

<template>
    <div class="flex justify-center items-center w-screen h-screen">
        <button class="btn btn-active btn-primary" @click="send_trigger">Benachrichigung senden</button>
    </div>
</template>