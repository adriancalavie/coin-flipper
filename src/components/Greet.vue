<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tossedMsg = ref("");
const tossValue = ref(1);

async function tossCoin() {
  tossedMsg.value = await invoke("process_toss");
}
</script>

<template>
  <div class="card">
    <button type="button" @click="tossCoin()">Toss</button>
  </div>

  <p>{{ tossedMsg }}</p>
</template>
