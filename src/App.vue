<template>
    <q-layout view="hHh lpR fFf">
        <q-header elevated class="bg-primary text-white">
            <q-toolbar>
                <q-toolbar-title>
                    App-Launcher
                </q-toolbar-title>
                <q-space />
                <q-btn flat round dense icon="more_vert" />
            </q-toolbar>
        </q-header>
        <q-page-container>
            <q-page class="q-pa-md bg-white">
                <q-input v-model="search" placeholder="Search" filled dense debounce="300" />
                <q-list bordered separator>
                    <q-item v-for="app in apps" :key="app.path.toString" clickable @click="launchApp(app)">
                        <q-item-section avatar>
                            <q-icon name="apps" />
                        </q-item-section>
                        <q-item-section>
                            {{ app.name }}
                        </q-item-section>
                    </q-item>
                </q-list>
            </q-page>
        </q-page-container>
    </q-layout>
</template>

<script lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'

interface App {
    name: String,
    path: String
}

export default {
    name: "App",
    data() {
        return {
            apps: [] as App[],
            search: ''
        }
    },
    methods: {
        async getInstalledApps(searchTerm: string = '') {
            try {
                this.apps = await invoke('get_installed_apps', { searchTerm })
            } catch (error) {
                console.error("Error retrieving installed apps:", error)
            }
        },
        launchApp(app: App) {
            invoke('launch_app', { app })
                .catch(console.error)
                .then(_onfullfilled => {
                    // appWindow.close()
                })
        }
    },
    computed: {
    },
    watch: {
        search(newSearch: string) {
            this.getInstalledApps(newSearch)
        }
    },
    mounted() {
        appWindow.center()
            .catch(console.error)
    }
}
</script>