<script setup lang="ts">
import { ref, watch } from 'vue'
import { get, set } from '@/helpers/settings'
import { Toggle } from '@nml/ui'

const settings = ref(await get())

watch(
  settings,
  async () => {
    await set(settings.value)
  },
  { deep: true },
)
</script>

<template>
  <div class="mt-4 flex items-center justify-between gap-4">
    <div>
      <h2 class="m-0 text-lg font-extrabold text-contrast">Discord RPC</h2>
      <p class="m-0 text-sm">
        Manages the Discord Rich Presence integration. Disabling this will cause 'Modrinth' to no
        longer show up as a game or app you are using on your Discord profile.
      </p>
      <p class="m-0 mt-2 text-sm">
        Note: This will not prevent any instance-specific Discord Rich Presence integrations, such
        as those added by mods. (app restart required to take effect)
      </p>
    </div>
    <Toggle
      id="disable-discord-rpc"
      v-model="settings.discord_rpc"
      :checked="settings.discord_rpc"
    />
  </div>
</template>
