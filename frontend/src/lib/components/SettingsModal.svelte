<script lang="ts">
  import { getAiSettings, saveAiSettings, askAi, type AiSettings } from "$lib/api";
  import { onMount } from "svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let settings = $state<AiSettings>({
    base_url: "http://localhost:20128/v1",
    api_key: "",
    model: "gpt-4o",
  });

  let saving = $state(false);
  let testing = $state(false);
  let testResult = $state<string | null>(null);
  let testSuccess = $state<boolean | null>(null);

  onMount(async () => {
    try {
      const s = await getAiSettings();
      settings = s;
    } catch (e) {
      console.error(e);
    }
  });

  async function handleSave() {
    saving = true;
    try {
      await saveAiSettings(settings);
      onClose();
    } catch (e) {
      alert(`Gagal menyimpan: ${e}`);
    } finally {
      saving = false;
    }
  }

  async function handleTest() {
    testing = true;
    testResult = null;
    testSuccess = null;
    try {
      await saveAiSettings(settings);
      const res = await askAi({
        user_prompt: "Tes koneksi: Jawab 'CACash AI Siap' dalam 1 baris.",
      });
      testResult = res.text.trim();
      testSuccess = true;
    } catch (e: any) {
      testResult = `Error: ${e}`;
      testSuccess = false;
    } finally {
      testing = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-2xl w-full max-w-md p-6 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm flex items-center gap-2">
          <span>⚙️</span> Pengaturan Finansial & AI
        </h3>
        <button onclick={onClose} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <div class="space-y-3.5 text-xs">
        <div>
          <label for="settings-base-url-input" class="block text-[#D1D5DB] font-medium mb-1">Base URL (OpenAI-Compatible)</label>
          <input
            id="settings-base-url-input"
            type="text"
            bind:value={settings.base_url}
            placeholder="http://localhost:20128/v1"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
          <span class="text-[10px] text-[#6B7280] mt-0.5 block">9Router lokal default: http://localhost:20128/v1</span>
        </div>

        <div>
          <label for="settings-model-input" class="block text-[#D1D5DB] font-medium mb-1">Model Name</label>
          <input
            id="settings-model-input"
            type="text"
            bind:value={settings.model}
            placeholder="gpt-4o / deepseek-chat"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        <div>
          <label for="settings-api-key-input" class="block text-[#D1D5DB] font-medium mb-1">API Key (Opsional jika 9Router tanpa auth)</label>
          <input
            id="settings-api-key-input"
            type="password"
            bind:value={settings.api_key}
            placeholder="sk-..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        {#if testResult}
          <div class="p-3 rounded-lg border text-xs {testSuccess ? 'bg-[#064E3B]/30 border-[#059669] text-[#34D399]' : 'bg-[#7F1D1D]/30 border-[#DC2626] text-[#F87171]'}">
            {testResult}
          </div>
        {/if}
      </div>

      <div class="pt-3 border-t border-[#272732] flex items-center justify-between">
        <button
          onclick={handleTest}
          disabled={testing}
          class="px-3 py-1.5 rounded-lg border border-[#374151] hover:bg-[#18181F] text-xs text-[#D1D5DB]"
        >
          {testing ? "Menguji..." : "Test AI"}
        </button>

        <div class="flex items-center gap-2">
          <button onclick={onClose} class="px-3 py-1.5 rounded text-xs text-[#9CA3AF] hover:bg-[#18181F]">Batal</button>
          <button onclick={handleSave} disabled={saving} class="px-4 py-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30">
            {saving ? "Menyimpan..." : "Simpan"}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
