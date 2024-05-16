<template>
    <q-header class="bg-primary text-white">
        <q-toolbar>
            <q-toolbar-title>
                App-Launcher
            </q-toolbar-title>
            <q-space />
            <q-btn flat round dense icon="settings" @click="showSettingsModal = true" />
        </q-toolbar>
    </q-header>
    <SettingsModal v-model:showSettings="showSettingsModal" @close="closeSettingsModal" />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import SettingsModal from './SettingsModal.vue'

export default defineComponent({
    name: 'AppHeader',
    components: {
        SettingsModal
    },
    data() {
        return {
            showSettingsModal: false
        }
    },
    methods: {
        closeSettingsModal() {
            this.showSettingsModal = false;
        }
    }
})
</script>