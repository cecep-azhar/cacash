<script lang="ts">
  import { askAi, type AskAiResponse } from "$lib/api";

  let prompt = $state("");
  let messages = $state<Array<{ role: "user" | "assistant"; text: string }>>([
    {
      role: "assistant",
      text: "Assalamu'alaikum Prof. Cecep. Saya AI Financial Coach CACash. Ada pertanyaan seputar optimasi pos belanja keluarga, percepatan pelunasan hutang, atau alokasi zakat dan investasi?",
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
          text: `Maaf, gagal memproses konsultasi: ${e}. Pastikan 9Router atau OpenAI proxy lokal aktif di port 20128.`,
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

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm flex items-center gap-2">
        <span>🤖</span> AI Financial Coach (Local-First)
      </h2>
      <span class="text-[10px] text-[#9CA3AF]">Privasi penuh: data finansial di-anonymize sebelum dikirim ke AI</span>
    </div>

    <span class="text-[10px] text-[#34D399] px-2.5 py-1 rounded bg-[#064E3B]/30 border border-[#059669]">
      Strict Religious Guardrails Active
    </span>
  </div>

  <!-- Messages List -->
  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-4xl mx-auto w-full space-y-4">
    {#each messages as m}
      <div class="flex flex-col {m.role === 'user' ? 'items-end' : 'items-start'}">
        <div class="max-w-[85%] rounded-2xl p-4 text-xs leading-relaxed {m.role === 'user' ? 'bg-[#10B981] text-white shadow-md' : 'bg-[#121217] border border-[#272732] text-[#E5E7EB]'}">
          <p class="whitespace-pre-wrap">{m.text}</p>
        </div>
        <span class="text-[9px] text-[#6B7280] mt-1 px-1">
          {m.role === 'user' ? 'Anda' : 'CACash Coach AI'}
        </span>
      </div>
    {/each}

    {#if loading}
      <div class="flex items-center gap-2 text-xs text-[#9CA3AF] italic">
        <span class="animate-spin">⏳</span> Menganalisis kondisi finansial keluarga...
      </div>
    {/if}
  </div>

  <!-- Quick Questions Suggestions -->
  <div class="px-6 md:px-8 py-2 max-w-4xl mx-auto w-full flex items-center gap-2 overflow-x-auto text-[11px] shrink-0">
    <button
      onclick={() => setPromptQuick("Berapa alokasi dana darurat ideal untuk keluarga dengan 2 anak?")}
      class="px-2.5 py-1 rounded-full bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-[#9CA3AF] hover:text-white shrink-0 transition-colors"
    >
      💡 Dana Darurat Ideal
    </button>
    <button
      onclick={() => setPromptQuick("Bagaimana strategi melunasi hutang dengan metode Debt Snowball?")}
      class="px-2.5 py-1 rounded-full bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-[#9CA3AF] hover:text-white shrink-0 transition-colors"
    >
      🎯 Lunasi Hutang (Snowball)
    </button>
    <button
      onclick={() => setPromptQuick("Bagaimana aturan pemisahan harta suami dan istri dalam Islam?")}
      class="px-2.5 py-1 rounded-full bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-[#9CA3AF] hover:text-white shrink-0 transition-colors"
    >
      🕌 Hak Harta Suami-Istri
    </button>
  </div>

  <!-- Chat Input -->
  <div class="p-4 md:px-8 border-t border-[#272732] bg-[#121217] shrink-0">
    <div class="max-w-4xl mx-auto flex items-center gap-3">
      <input
        type="text"
        bind:value={prompt}
        onkeydown={(e) => e.key === 'Enter' && handleSend()}
        placeholder="Tanyakan saran pengelolaan keuangan keluarga..."
        class="flex-1 bg-[#18181F] border border-[#272732] rounded-xl px-4 py-3 text-xs text-white focus:border-[#10B981] focus:outline-none"
      />
      <button
        onclick={handleSend}
        disabled={loading || !prompt.trim()}
        class="px-5 py-3 rounded-xl bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30 transition-all disabled:opacity-50"
      >
        Kirim
      </button>
    </div>
  </div>
</div>
