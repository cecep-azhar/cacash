<script lang="ts">
  import { askAi, type AskAiResponse } from "$lib/api";

  let isOpen = $state(false);
  let isMinimized = $state(false);
  let promptText = $state("");
  let isGenerating = $state(false);

  interface ChatMessage {
    id: string;
    role: "user" | "assistant";
    content: string;
    timestamp: number;
  }

  let messages = $state<ChatMessage[]>([
    {
      id: "msg_welcome",
      role: "assistant",
      content:
        "Assalamu'alaikum Prof. Cecep! Saya Financial Coach CACash. Siap membantu audit kas keluarga, pos dana darurat, zakat, maupun strategi debt snowball tanpa riba.",
      timestamp: Date.now(),
    },
  ]);

  const quickPrompts = [
    {
      label: "🛡️ Dana Darurat",
      prompt: "Berapa alokasi dana darurat ideal untuk keluarga dan di mana instrumen penyimpanannya?",
    },
    {
      label: "⚡ Snowball Hutang",
      prompt: "Bagaimana urutan prioritas pelunasan hutang dengan metode Debt Snowball tanpa riba?",
    },
    {
      label: "🕌 Zakat & Nisab",
      prompt: "Kapan zakat mal wajib dikeluarkan dan bagaimana perhitungannya saat ini?",
    },
    {
      label: "📈 Formula 50/30/20",
      prompt: "Bagaimana cara menerapkan prinsip anggaran 50/30/20 disesuaikan dengan nafkah syariah?",
    },
  ];

  async function handleSend(customText?: string) {
    const textToSend = customText || promptText.trim();
    if (!textToSend || isGenerating) return;

    const userMsg: ChatMessage = {
      id: `usr_${Date.now()}`,
      role: "user",
      content: textToSend,
      timestamp: Date.now(),
    };
    messages.push(userMsg);
    promptText = "";
    isGenerating = true;

    try {
      const res: AskAiResponse = await askAi({
        user_prompt: textToSend,
      });

      const aiMsg: ChatMessage = {
        id: `ai_${Date.now()}`,
        role: "assistant",
        content: res.text,
        timestamp: Date.now(),
      };
      messages.push(aiMsg);
    } catch (e: any) {
      messages.push({
        id: `err_${Date.now()}`,
        role: "assistant",
        content: `Maaf, gagal menghubungi AI Coach: ${e}. Pastikan service AI lokal/proxy aktif.`,
        timestamp: Date.now(),
      });
    } finally {
      isGenerating = false;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      handleSend();
    }
  }

  function copyText(txt: string) {
    navigator.clipboard.writeText(txt);
  }
</script>

<!-- Floating Action Button (Closed State) -->
{#if !isOpen}
  <button
    type="button"
    onclick={() => (isOpen = true)}
    class="fixed bottom-5 right-5 z-50 flex items-center gap-2.5 px-4 py-2.5 rounded-full bg-[#121217] hover:bg-[#18181F] text-white border border-[#10B981]/50 shadow-2xl shadow-[#10B981]/20 hover:scale-105 active:scale-95 transition-all text-xs font-semibold cursor-pointer group"
    title="Buka AI Financial Coach"
  >
    <span class="w-2 h-2 rounded-full bg-[#10B981] animate-pulse"></span>
    <span class="bg-gradient-to-r from-[#A7F3D0] to-[#10B981] bg-clip-text text-transparent font-bold">
      ✨ AI Financial Coach
    </span>
    <span class="px-1.5 py-0.5 rounded text-[10px] font-mono bg-[#10B981]/20 text-[#6EE7B7] border border-[#10B981]/30">
      CADS
    </span>
  </button>
{:else}
  <!-- Floating Smart Card Panel -->
  <div
    class="fixed bottom-4 right-4 z-50 w-96 max-w-[calc(100vw-2rem)] bg-[#121217] border border-[#272732] shadow-2xl shadow-black/80 rounded-2xl flex flex-col overflow-hidden font-sans transition-all {isMinimized ? 'h-14' : 'h-[540px]'}"
  >
    <!-- Card Header -->
    <div
      class="h-14 border-b border-[#272732] px-4 bg-[#18181F] flex items-center justify-between shrink-0 select-none cursor-pointer"
      onclick={() => (isMinimized = !isMinimized)}
      role="button"
      tabindex="0"
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          isMinimized = !isMinimized;
        }
      }}
    >
      <div class="flex items-center gap-2.5">
        <div class="w-7 h-7 rounded-lg bg-[#10B981]/20 border border-[#10B981]/40 flex items-center justify-center text-xs text-[#34D399]">
          ✨
        </div>
        <div>
          <span class="font-bold text-white text-xs block leading-tight">AI Financial Coach</span>
          <span class="text-[10px] text-emerald-400 font-mono block">● Online (Local-First Guardrails)</span>
        </div>
      </div>

      <div class="flex items-center gap-1.5" onclick={(e) => e.stopPropagation()} role="presentation">
        <button
          type="button"
          onclick={() => (isMinimized = !isMinimized)}
          class="w-7 h-7 rounded-lg hover:bg-[#272732] text-[#9CA3AF] hover:text-white flex items-center justify-center text-xs transition-colors"
          title={isMinimized ? "Perbesar Card" : "Kecilkan Card"}
        >
          {isMinimized ? "▲" : "▼"}
        </button>
        <button
          type="button"
          onclick={() => (isOpen = false)}
          class="w-7 h-7 rounded-lg hover:bg-rose-500/20 hover:text-rose-400 text-[#9CA3AF] flex items-center justify-center text-xs transition-colors"
          title="Tutup Card"
        >
          ✕
        </button>
      </div>
    </div>

    <!-- Card Body & Chat Stream -->
    {#if !isMinimized}
      <div class="flex-1 flex flex-col min-h-0 bg-[#0E0E12]">
        <!-- Messages Area -->
        <div class="flex-1 overflow-y-auto p-4 space-y-3.5 text-xs">
          {#each messages as m}
            <div class="flex flex-col {m.role === 'user' ? 'items-end' : 'items-start'}">
              <div
                class="max-w-[85%] rounded-xl p-3 leading-relaxed relative group {m.role === 'user' ? 'bg-[#10B981] text-white' : 'bg-[#18181F] border border-[#272732] text-[#E5E7EB]'}"
              >
                <p class="whitespace-pre-wrap">{m.content}</p>
                {#if m.role === 'assistant'}
                  <button
                    type="button"
                    onclick={() => copyText(m.content)}
                    class="absolute -bottom-2 right-2 opacity-0 group-hover:opacity-100 px-1.5 py-0.5 rounded text-[9px] bg-[#272732] text-[#9CA3AF] hover:text-white transition-opacity"
                    title="Salin Pesan"
                  >
                    Salin
                  </button>
                {/if}
              </div>
              <span class="text-[9px] text-[#6B7280] mt-1 px-1">
                {m.role === 'user' ? 'Anda' : 'Coach AI'} • {new Date(m.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
              </span>
            </div>
          {/each}

          {#if isGenerating}
            <div class="flex items-center gap-2 p-3 bg-[#18181F] border border-[#272732] rounded-xl text-xs text-[#9CA3AF] w-fit">
              <span class="w-2 h-2 rounded-full bg-[#10B981] animate-ping"></span>
              <span>Menganalisis kondisi kas keluarga...</span>
            </div>
          {/if}
        </div>

        <!-- Quick Prompts Pill Row -->
        <div class="px-3 py-2 border-t border-[#272732] bg-[#121217] flex items-center gap-1.5 overflow-x-auto scrollbar-none shrink-0">
          {#each quickPrompts as qp}
            <button
              type="button"
              onclick={() => handleSend(qp.prompt)}
              disabled={isGenerating}
              class="px-2.5 py-1 rounded-full text-[10px] bg-[#18181F] hover:bg-[#272732] text-[#9CA3AF] hover:text-[#6EE7B7] border border-[#272732] hover:border-[#10B981]/40 shrink-0 transition-colors disabled:opacity-50"
            >
              {qp.label}
            </button>
          {/each}
        </div>

        <!-- Input Area -->
        <div class="p-3 border-t border-[#272732] bg-[#16161D] shrink-0">
          <div class="relative flex items-center">
            <textarea
              bind:value={promptText}
              onkeydown={handleKeyDown}
              placeholder="Tanya coach finansial... (Ctrl+Enter)"
              rows="2"
              class="w-full bg-[#18181F] border border-[#272732] focus:border-[#10B981] rounded-xl pl-3 pr-10 py-2 text-xs text-white placeholder-[#6B7280] focus:outline-none resize-none transition-colors"
            ></textarea>
            <button
              type="button"
              onclick={() => handleSend()}
              disabled={!promptText.trim() || isGenerating}
              class="absolute right-2.5 bottom-2.5 p-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white disabled:opacity-30 disabled:hover:bg-[#10B981] transition-all cursor-pointer"
              title="Kirim Pesan"
            >
              <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <line x1="22" y1="2" x2="11" y2="13"></line>
                <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
              </svg>
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>
{/if}
