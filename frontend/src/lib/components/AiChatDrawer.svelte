<script lang="ts">
  import { askAi, type AskAiResponse } from "$lib/api";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let prompt = $state("");
  let messages = $state<Array<{ role: "user" | "assistant"; text: string }>>([
    {
      role: "assistant",
      text: "Assalamu'alaikum. Saya AI Financial Coach CACash. Ada yang ingin dikonsultasikan seputar pos anggaran keluarga, percepatan pelunasan hutang, atau zakat?",
    },
  ]);
  let loading = $state(false);

  async function handleSend() {
    if (!prompt.trim() || loading) return;
    const userText = prompt.trim();
    prompt = "";
    messages = [...messages, { role: "user", text: userText }];

    loading = true;
    try {
      const res: AskAiResponse = await askAi({ user_prompt: userText });
      messages = [...messages, { role: "assistant", text: res.text }];
    } catch (e: any) {
      messages = [
        ...messages,
        {
          role: "assistant",
          text: `Gagal memproses konsultasi: ${e}. Pastikan 9Router lokal aktif di port 20128.`,
        },
      ];
    } finally {
      loading = false;
    }
  }

  function setPromptQuick(q: string) {
    prompt = q;
  }
</script>

{#if isOpen}
  <!-- Backdrop for small screens -->
  <button
    type="button"
    onclick={onClose}
    class="fixed inset-0 z-40 bg-black/50 backdrop-blur-xs md:hidden transition-opacity border-0 p-0 cursor-default"
    aria-label="Tutup Panel AI"
  ></button>

  <!-- Side Drawer Panel -->
  <aside
    class="fixed inset-y-0 right-0 z-50 w-96 max-w-[90vw] bg-[#121217] border-l border-[#272732] flex flex-col justify-between shadow-2xl transition-transform duration-200 select-none"
  >
    <!-- Drawer Header -->
    <div class="h-12 border-b border-[#272732] px-4 flex items-center justify-between bg-[#16161D] shrink-0">
      <div class="flex items-center gap-2">
        <div class="w-6 h-6 rounded-md bg-[#10B981]/20 border border-[#10B981]/40 flex items-center justify-center text-xs text-[#34D399]">
          <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2 2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z"></path>
            <rect x="4" y="8" width="16" height="12" rx="2"></rect>
          </svg>
        </div>
        <span class="font-bold text-white text-xs">AI Financial Coach</span>
        <span class="text-[9px] px-1.5 py-0.5 rounded bg-[#10B981]/20 text-[#34D399] font-mono">Local</span>
      </div>

      <button
        onclick={onClose}
        class="p-1 rounded text-[#9CA3AF] hover:text-white hover:bg-[#272732] transition-colors"
        title="Tutup Panel"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>

    <!-- Chat Messages -->
    <div class="flex-1 overflow-y-auto p-4 space-y-3.5 text-xs">
      {#each messages as m}
        <div class="flex flex-col {m.role === 'user' ? 'items-end' : 'items-start'}">
          <div
            class="max-w-[88%] rounded-xl p-3 leading-relaxed {m.role === 'user' ? 'bg-[#10B981] text-white shadow-sm' : 'bg-[#18181F] border border-[#272732] text-[#E5E7EB]'}"
          >
            <p class="whitespace-pre-wrap">{m.text}</p>
          </div>
          <span class="text-[9px] text-[#6B7280] mt-1 px-1">
            {m.role === 'user' ? 'Anda' : 'CACash Coach'}
          </span>
        </div>
      {/each}

      {#if loading}
        <div class="flex items-center gap-2 text-xs text-[#9CA3AF] italic">
          <span class="animate-spin text-[#10B981]">⟳</span> Menghitung analisis finansial...
        </div>
      {/if}
    </div>

    <!-- Quick Prompts -->
    <div class="px-4 py-2 border-t border-[#272732] flex gap-1.5 overflow-x-auto text-[10px] shrink-0 bg-[#16161D]">
      <button
        onclick={() => setPromptQuick("Alokasi dana darurat ideal keluarga?")}
        class="px-2 py-1 rounded-md bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-[#9CA3AF] hover:text-white shrink-0"
      >
        Dana Darurat
      </button>
      <button
        onclick={() => setPromptQuick("Bagaimana metode melunasi hutang Debt Snowball?")}
        class="px-2 py-1 rounded-md bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-[#9CA3AF] hover:text-white shrink-0"
      >
        Debt Snowball
      </button>
      <button
        onclick={() => setPromptQuick("Kapan zakat mal wajib dibayarkan?")}
        class="px-2 py-1 rounded-md bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-[#9CA3AF] hover:text-white shrink-0"
      >
        Zakat Mal
      </button>
    </div>

    <!-- Chat Input -->
    <div class="p-3 border-t border-[#272732] bg-[#16161D] shrink-0">
      <div class="flex items-center gap-2">
        <input
          type="text"
          bind:value={prompt}
          onkeydown={(e) => e.key === 'Enter' && handleSend()}
          placeholder="Tanya coach finansial..."
          class="flex-1 bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
        />
        <button
          onclick={handleSend}
          disabled={loading || !prompt.trim()}
          class="px-3 py-2 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold transition-all disabled:opacity-40"
        >
          Kirim
        </button>
      </div>
    </div>
  </aside>
{/if}
