<template>
    <q-layout view="hHh lpR fFf">
        <AppHeader @update-open-shortcut="updateOpenShortcut" />
        <q-page-container>
            <q-page class="q-pa-md">
                <q-input ref="search" v-model="search" placeholder="Search" filled dense autofocus />
                <q-list bordered separator>
                    <AppItem v-for="(app, _index) in filteredApps" :key="app.path.toString()" :app="app"
                        @clearSearch="search = ''" />
                </q-list>
            </q-page>
        </q-page-container>
    </q-layout>
</template>

<script lang="ts">
import App from './types/App'
import AppHeader from './components/AppHeader.vue'
import AppItem from './components/AppItem.vue'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'
import { isRegistered, register, unregister } from '@tauri-apps/api/globalShortcut'
import Fuse from 'fuse.js'

export default {
    name: "App",
    components: {
        AppHeader,
        AppItem
    },
    data() {
        return {
            apps: [] as App[],
            search: '',
            currentOpenShortcut: ''
        }
    },
    methods: {
        async getInstalledApps() {
            this.apps = await invoke('get_installed_apps')
        },
        async updateOpenShortcut(newShortcut: string) {
            if (this.currentOpenShortcut) {
                await unregister(this.currentOpenShortcut)
            }
            await register(newShortcut, async () => {
                if (await appWindow.isVisible()) {
                    await appWindow.hide();
                    this.search = ''
                    window.scrollTo(0, 0)
                } else {
                    await appWindow.show();
                    await appWindow.setFocus();
                    (this.$refs.search as HTMLInputElement).focus()
                }
            })
            this.currentOpenShortcut = newShortcut
        },
        async registerCloseShortcut() {
            if (!await isRegistered('Alt+F4')) {
                await register('Alt+F4', async () => {
                    await appWindow.close()
                })
            }
        },
        async registerSearchShortcut() {
            if (!await isRegistered('Ctrl+Space')) {
                await register('Ctrl+Space', async () => {
                    const searchInput = this.$refs.search as HTMLInputElement
                    searchInput.focus()
                    searchInput.select()
                    window.scrollTo(0, 0)
                })
            }
        }
    },
    computed: {
        filteredApps(): App[] {
            const fuse = new Fuse(this.apps, {
                keys: ['name'],
                includeScore: true,
                shouldSort: true,
                includeMatches: true,
                threshold: 0.5,
                distance: 1000,
                // useExtendedSearch: true
            })
            return this.search ? fuse.search(this.search).map(result => result.item) : this.apps
        }
    },
    mounted() {
        this.getInstalledApps()
        appWindow.center()
        this.registerCloseShortcut()
        this.registerSearchShortcut()
    }
}
</script>

<style>
::-webkit-scrollbar {
    display: none;
}
</style>