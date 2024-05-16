<template>
    <q-header elevated class="bg-primary text-white">
        <q-toolbar>
            <q-toolbar-title>
                App-Launcher
            </q-toolbar-title>
            <q-space />
            <q-btn flat round dense icon="more_vert" />
        </q-toolbar>
    </q-header>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
    name: 'AppHeader'
})
</script>